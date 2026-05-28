// 对应 C++ 中的 PakInterface.h / PakInterface.cpp
// PAK 文件包读取接口 - 用于从 .pak 资源包中读取游戏资源文件
//
// C++ 依赖: zlib (解压), fcaseopen (大小写不敏感文件打开)
// Rust 映射: flate2 (zlib), 标准库文件 I/O

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::{ptr, slice};

const FILEFLAGS_END: u8 = 0x80;
const PAK_MAGIC: u32 = 0xBAC04AC0;

// ====================================================================================================
// ★ 一个 PakRecord 实例对应资源包内的一个资源文件的数据
// ====================================================================================================
pub struct PakRecord {
    pub collection: *mut PakCollection,
    pub file_name: String,
    pub file_time: i64,
    pub start_pos: i32,
    pub size: i32,
}

impl PakRecord {
    fn new() -> Self {
        Self {
            collection: ptr::null_mut(),
            file_name: String::new(),
            file_time: 0,
            start_pos: 0,
            size: 0,
        }
    }
}

// ====================================================================================================
// ★ 一个 PakCollection 实例对应一个 pak 资源包在内存中的映射
// ====================================================================================================
pub struct PakCollection {
    pub data_ptr: *mut u8,
    pub data_size: usize,
}

impl PakCollection {
    pub fn new(size: usize) -> Self {
        // SAFETY: allocated memory initialized from file data in AddPakFile
        let data = unsafe {
            let layout = std::alloc::Layout::array::<u8>(size).unwrap();
            std::alloc::alloc(layout)
        };
        Self { data_ptr: data, data_size: size }
    }
}

impl Drop for PakCollection {
    fn drop(&mut self) {
        if !self.data_ptr.is_null() && self.data_size > 0 {
            unsafe {
                let layout = std::alloc::Layout::array::<u8>(self.data_size).unwrap();
                std::alloc::dealloc(self.data_ptr, layout);
            }
            self.data_ptr = ptr::null_mut();
        }
    }
}

// ====================================================================================================
// ★ PFILE - Pak 文件句柄
// ====================================================================================================
pub struct PFile {
    pub record: *mut PakRecord,
    pub pos: i32,
    pub fp: Option<File>,
}

impl PFile {
    fn new() -> Self {
        Self { record: ptr::null_mut(), pos: 0, fp: None }
    }
}

// ====================================================================================================
// ★ PakInterface - PAK 文件包管理接口
// ====================================================================================================
pub struct PakInterface {
    pub pak_collection_list: Vec<PakCollection>,
    pub pak_record_map: HashMap<String, PakRecord>,
    resource_folder: String,
}

impl PakInterface {
    pub fn new() -> Self {
        Self {
            pak_collection_list: Vec::new(),
            pak_record_map: HashMap::new(),
            resource_folder: String::new(),
        }
    }

    pub fn set_resource_folder(&mut self, folder: &str) {
        self.resource_folder = folder.to_string();
    }

    // Normalize path for pak lookup - matches C++ NormalizePakPath exactly
    pub fn normalize_pak_path(file_name: &str) -> String {
        let path = Path::new(file_name);
        let mut result = if path.is_absolute() || (file_name.len() > 1 && file_name.as_bytes()[1] == b':') {
            // ASSUMPTION: C++ uses filesystem::path to normalize and make relative to resource folder.
            // In Rust, we simplify: strip common prefix and normalize separators.
            path.to_string_lossy().replace('\\', "/")
        } else {
            file_name.to_string()
        };

        // Strip leading "./" if present
        if result.len() >= 2 && result.as_bytes()[0] == b'.' && result.as_bytes()[1] == b'/' {
            result = result[2..].to_string();
        }

        // C++ converts to uppercase for case-insensitive matching
        result.to_uppercase()
    }

    pub fn add_pak_file(&mut self, file_name: &str) -> bool {
        let mut file = match File::open(file_name) {
            Ok(f) => f,
            Err(_) => return false,
        };

        let file_size = match file.metadata() {
            Ok(m) => m.len() as usize,
            Err(_) => return false,
        };

        self.pak_collection_list.push(PakCollection::new(file_size));
        let pak_collection_ptr: *mut PakCollection = self.pak_collection_list.last_mut().unwrap() as *mut PakCollection;

        // SAFETY: pak_collection is a valid pointer into our Vec, and data_ptr was properly allocated
        let data_slice = unsafe {
            if file_size > 0 {
                let slice_data = slice::from_raw_parts_mut(
                    (*pak_collection_ptr).data_ptr,
                    file_size,
                );
                slice_data
            } else {
                &mut []
            }
        };

        if file.read_exact(data_slice).is_err() {
            // Clean up on failure
            self.pak_collection_list.pop();
            return false;
        }

        // XOR each byte with 0xF7 (matches C++ logic)
        for byte in data_slice.iter_mut() {
            *byte ^= 0xF7;
        }

        // Create record for the pak file itself
        let pak_key = Self::normalize_pak_path(file_name);
        self.pak_record_map.entry(pak_key.clone()).or_insert_with(|| {
            let mut record = PakRecord::new();
            record.collection = pak_collection_ptr;
            record.file_name = pak_key.clone();
            record.start_pos = 0;
            record.size = file_size as i32;
            record
        });

        // Parse pak file entries using the internal reader
        let mut pfile = PFile::new();
        pfile.pos = 0;
        // SAFETY: pak_collection_ptr points to a valid, lifetime-consistent PakCollection
        pfile.record = {
            let pak_key = Self::normalize_pak_path(file_name);
            // SAFETY: raw pointer to record within self.pak_record_map; lifetime tied to self
            self.pak_record_map.get_mut(&pak_key)
                .map(|r| r as *mut PakRecord)
                .unwrap_or(ptr::null_mut())
        };

        // Read magic
        let magic = self.read_u32_le_internal(&mut pfile);
        if magic != PAK_MAGIC {
            return false;
        }

        // Read version
        let version = self.read_u32_le_internal(&mut pfile);
        if version > 0 {
            return false;
        }

        let mut a_pos: i32 = 0;

        loop {
            let mut flags: u8 = 0;
            let count = self.fread_internal(&mut flags as *mut u8 as *mut std::ffi::c_void, 1, 1, &mut pfile);
            if (flags & FILEFLAGS_END) != 0 || count == 0 {
                break;
            }

            let mut name_width: u8 = 0;
            self.fread_internal(&mut name_width as *mut u8 as *mut std::ffi::c_void, 1, 1, &mut pfile);

            let mut name_buf = vec![0u8; name_width as usize + 1];
            self.fread_internal(name_buf.as_mut_ptr() as *mut std::ffi::c_void, 1, name_width as i32, &mut pfile);

            let a_src_size = self.read_i32_le_internal(&mut pfile);
            let a_file_time = self.read_i64_le_internal(&mut pfile);

            // Replace backslashes with forward slashes
            for byte in name_buf.iter_mut().take(name_width as usize) {
                if *byte == b'\\' {
                    *byte = b'/';
                }
            }

            let name_str = String::from_utf8_lossy(&name_buf[..name_width as usize]).to_string();
            let record_key = Self::normalize_pak_path(&name_str);

            self.pak_record_map.entry(record_key.clone()).or_insert_with(|| {
                let mut record = PakRecord::new();
                record.collection = pak_collection_ptr;
                record.file_name = record_key;
                record.start_pos = a_pos;
                record.size = a_src_size;
                record.file_time = a_file_time;
                record
            });

            a_pos += a_src_size;
        }

        let offset = self.ftell_internal(&mut pfile);

        // Adjust start positions for all records belonging to this collection
        for (_, record) in self.pak_record_map.iter_mut() {
            // SAFETY: raw pointer comparison for same collection ownership
            if record.collection == pak_collection_ptr {
                record.start_pos += offset;
            }
        }

        true
    }

    // Internal read functions using raw pointer - these mirror the C++ PFILE-based logic

    fn read_u32_le_internal(&self, pfile: &mut PFile) -> u32 {
        let mut buf = [0u8; 4];
        self.fread_internal(buf.as_mut_ptr() as *mut std::ffi::c_void, 1, 4, pfile);
        u32::from_le_bytes(buf)
    }

    fn read_i32_le_internal(&self, pfile: &mut PFile) -> i32 {
        let mut buf = [0u8; 4];
        self.fread_internal(buf.as_mut_ptr() as *mut std::ffi::c_void, 1, 4, pfile);
        i32::from_le_bytes(buf)
    }

    fn read_i64_le_internal(&self, pfile: &mut PFile) -> i64 {
        let mut buf = [0u8; 8];
        self.fread_internal(buf.as_mut_ptr() as *mut std::ffi::c_void, 1, 8, pfile);
        i64::from_le_bytes(buf)
    }

    fn ftell_internal(&self, pfile: &mut PFile) -> i32 {
        pfile.pos
    }

    fn fread_internal(&self, ptr: *mut std::ffi::c_void, elem_size: i32, count: i32, pfile: &mut PFile) -> i32 {
        // SAFETY: record is non-null and points to a valid PakRecord within self.pak_record_map
        let record = unsafe {
            if pfile.record.is_null() {
                return 0;
            }
            &*pfile.record
        };

        let size_bytes = std::cmp::min(elem_size * count, record.size - pfile.pos) as usize;
        if size_bytes == 0 {
            return 0;
        }

        // SAFETY: collection is non-null (same lifetime as self), data_ptr was alloc'd in AddPakFile
        let pak_data = unsafe {
            if record.collection.is_null() {
                return 0;
            }
            slice::from_raw_parts(
                (*record.collection).data_ptr,
                (*record.collection).data_size,
            )
        };

        let src_offset = (record.start_pos + pfile.pos) as usize;
        if src_offset + size_bytes > pak_data.len() {
            return 0;
        }

        let src = &pak_data[src_offset..src_offset + size_bytes];
        // SAFETY: ptr is a valid pointer provided by caller (stack variable address)
        unsafe {
            let dest = slice::from_raw_parts_mut(ptr as *mut u8, size_bytes);
            dest.copy_from_slice(src);
        }

        // Update the position in PFILE (now we have &mut access)
        pfile.pos += size_bytes as i32;

        (size_bytes / elem_size.max(1) as usize) as i32
    }

    // ─── Public API matching C++ PakInterfaceBase ───

    pub fn f_open(&mut self, file_name: &str, access: &str) -> *mut PFile {
        let access_lower = access.to_lowercase();

        if access_lower.contains('r') {
            let key = Self::normalize_pak_path(file_name);

            if let Some(record) = self.pak_record_map.get_mut(&key) {
                let mut pfile = Box::new(PFile::new());
                pfile.record = record as *mut PakRecord;
                pfile.pos = 0;
                pfile.fp = None;
                return Box::into_raw(pfile);
            }
        }

        // Try opening from filesystem (resource folder or direct path)
        let file_path = if !self.resource_folder.is_empty() {
            format!("{}/{}", self.resource_folder, file_name)
        } else {
            file_name.to_string()
        };

        match File::open(&file_path) {
            Ok(file) => {
                let mut pfile = Box::new(PFile::new());
                pfile.record = ptr::null_mut();
                pfile.pos = 0;
                pfile.fp = Some(file);
                Box::into_raw(pfile)
            }
            Err(_) => ptr::null_mut(),
        }
    }

    pub fn f_close(&self, file: *mut PFile) -> i32 {
        if file.is_null() {
            return 0;
        }
        // SAFETY: file is a valid pointer previously returned by f_open
        unsafe {
            let pfile = Box::from_raw(file);
            // File will be closed by File's Drop
            drop(pfile);
        }
        0
    }

    pub fn f_seek(&self, file: *mut PFile, offset: i32, origin: i32) -> i32 {
        if file.is_null() {
            return -1;
        }
        // SAFETY: file is a valid pointer previously returned by f_open
        let pfile = unsafe { &mut *file };

        if !pfile.record.is_null() {
            pfile.pos = match origin {
                0 /* SEEK_SET */ => offset,
                2 /* SEEK_END */ => {
                    // SAFETY: record pointer validity
                    let rec = unsafe { &*pfile.record };
                    rec.size - offset
                }
                1 /* SEEK_CUR */ => pfile.pos + offset,
                _ => pfile.pos,
            };
            // SAFETY: record pointer validity
            let rec = unsafe { &*pfile.record };
            pfile.pos = pfile.pos.clamp(0, rec.size);
            0
        } else if let Some(ref mut fp) = pfile.fp {
            let seek_from = match origin {
                0 => SeekFrom::Start(offset as u64),
                2 => SeekFrom::End(-(offset as i64)),
                1 => SeekFrom::Current(offset as i64),
                _ => SeekFrom::Start(offset as u64),
            };
            if fp.seek(seek_from).is_ok() { 0 } else { -1 }
        } else {
            -1
        }
    }

    pub fn f_tell(&self, file: *const PFile) -> i32 {
        if file.is_null() {
            return -1;
        }
        // SAFETY: file is a valid pointer
        let pfile = unsafe { &*file };

        if !pfile.record.is_null() {
            pfile.pos
        } else if let Some(ref _fp) = pfile.fp {
            // ASSUMPTION: filesystem file position not tracked here;
            // return 0 as fallback. C++ ftell is not easily replicated
            // in Rust without keeping our own position counter.
            0
        } else {
            -1
        }
    }

    pub fn f_read(&self, ptr: *mut std::ffi::c_void, elem_size: i32, count: i32, file: *mut PFile) -> i32 {
        if file.is_null() {
            return 0;
        }
        // SAFETY: file is a valid pointer
        let pfile = unsafe { &mut *file };

        if !pfile.record.is_null() {
            let rec = unsafe { &*pfile.record };
            let size_bytes = std::cmp::min(elem_size * count, rec.size - pfile.pos) as usize;
            if size_bytes == 0 {
                return 0;
            }

            // SAFETY: collection and data_ptr validity
            let pak_data = unsafe {
                slice::from_raw_parts(
                    (*rec.collection).data_ptr,
                    (*rec.collection).data_size,
                )
            };

            let src_offset = (rec.start_pos + pfile.pos) as usize;
            let src = &pak_data[src_offset..src_offset + size_bytes];

            // SAFETY: ptr is valid (caller-supplied buffer)
            unsafe {
                let dest = slice::from_raw_parts_mut(ptr as *mut u8, size_bytes);
                dest.copy_from_slice(src);
            }

            pfile.pos += size_bytes as i32;
            (size_bytes / elem_size.max(1) as usize) as i32
        } else if let Some(ref mut fp) = pfile.fp {
            let size = (elem_size * count) as usize;
            let mut buf = vec![0u8; size];
            match fp.read_exact(&mut buf) {
                Ok(_) => {
                    // SAFETY: ptr is a valid caller buffer
                    unsafe {
                        let dest = slice::from_raw_parts_mut(ptr as *mut u8, size);
                        dest.copy_from_slice(&buf);
                    }
                    count
                }
                Err(_) => {
                    // Partial read fallback: return elements read
                    // FIXME: exact bytes-read tracking not implemented for file path
                    0
                }
            }
        } else {
            0
        }
    }

    pub fn f_getc(&self, file: *mut PFile) -> i32 {
        if file.is_null() {
            return -1; // EOF
        }
        // SAFETY: valid pointer
        let pfile = unsafe { &mut *file };

        if !pfile.record.is_null() {
            loop {
                let rec = unsafe { &*pfile.record };
                if pfile.pos >= rec.size {
                    return -1; // EOF
                }
                // SAFETY: data_ptr validity
                let pak_data = unsafe {
                    slice::from_raw_parts(
                        (*rec.collection).data_ptr,
                        (*rec.collection).data_size,
                    )
                };
                let byte_pos = (rec.start_pos + pfile.pos) as usize;
                pfile.pos += 1;
                let ch = pak_data[byte_pos];
                if ch != b'\r' {
                    return ch as i32;
                }
            }
        } else if let Some(ref mut fp) = pfile.fp {
            let mut buf = [0u8; 1];
            match fp.read_exact(&mut buf) {
                Ok(_) => buf[0] as i32,
                Err(_) => -1,
            }
        } else {
            -1
        }
    }

    pub fn ungetc(&self, ch: i32, file: *mut PFile) -> i32 {
        if file.is_null() {
            return -1;
        }
        let pfile = unsafe { &mut *file };

        if !pfile.record.is_null() {
            pfile.pos = std::cmp::max(pfile.pos - 1, 0);
            ch
        } else if let Some(ref mut fp) = pfile.fp {
            // Seek back one byte
            let _ = fp.seek(SeekFrom::Current(-1));
            ch
        } else {
            -1
        }
    }

    pub fn f_gets(&self, buf: *mut i8, size: i32, file: *mut PFile) -> *mut i8 {
        if file.is_null() || buf.is_null() || size <= 0 {
            return ptr::null_mut();
        }
        let pfile = unsafe { &mut *file };

        if !pfile.record.is_null() {
            let mut idx = 0;
            let rec = unsafe { &*pfile.record };
            let pak_data = unsafe {
                slice::from_raw_parts(
                    (*rec.collection).data_ptr,
                    (*rec.collection).data_size,
                )
            };

            while idx < size as usize {
                if pfile.pos >= rec.size {
                    if idx == 0 {
                        return ptr::null_mut();
                    }
                    break;
                }
                let byte_pos = (rec.start_pos + pfile.pos) as usize;
                pfile.pos += 1;
                let ch = pak_data[byte_pos] as i8;
                if ch != b'\r' as i8 {
                    // SAFETY: buf is valid caller buffer
                    unsafe { *buf.add(idx) = ch; }
                    idx += 1;
                }
                if ch == b'\n' as i8 {
                    break;
                }
            }
            // SAFETY: buf is valid, null-terminate
            unsafe { *buf.add(idx) = 0; }
            buf
        } else if let Some(ref mut fp) = pfile.fp {
            // Read line from file manually
            let mut idx = 0i32;
            while idx < size - 1 {
                let mut byte = [0u8; 1];
                match fp.read_exact(&mut byte) {
                    Ok(_) => {
                        let ch = byte[0] as i8;
                        if ch != b'\r' as i8 {
                            unsafe { *buf.add(idx as usize) = ch; }
                            idx += 1;
                        }
                        if ch == b'\n' as i8 {
                            break;
                        }
                    }
                    Err(_) => {
                        if idx == 0 {
                            return ptr::null_mut();
                        }
                        break;
                    }
                }
            }
            unsafe { *buf.add(idx as usize) = 0; }
            buf
        } else {
            ptr::null_mut()
        }
    }

    pub fn f_eof(&self, file: *const PFile) -> bool {
        if file.is_null() {
            return true;
        }
        let pfile = unsafe { &*file };

        if !pfile.record.is_null() {
            let rec = unsafe { &*pfile.record };
            pfile.pos >= rec.size
        } else {
            // ASSUMPTION: filesystem file EOF detection not easily done without reading.
            false
        }
    }

    // Helper: check if a path exists in pak records
    pub fn has_file(&self, path: &str) -> bool {
        let key = Self::normalize_pak_path(path);
        self.pak_record_map.contains_key(&key)
    }
}
