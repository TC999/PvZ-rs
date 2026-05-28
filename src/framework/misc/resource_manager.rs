// 对应 C++ 中的 ResourceManager.h / ResourceManager.cpp
// 资源管理器

use std::collections::HashMap;
use std::fs;
use flate2::read::ZlibDecoder;
use std::io::Read;

pub struct ResourceFile {
    pub name: String,
    pub data: Vec<u8>,
}

pub struct ResourceManager {
    resources: HashMap<String, Vec<u8>>,
    search_paths: Vec<String>,
}

impl ResourceManager {
    pub fn new() -> Self {
        log::debug!("ResourceManager::new: 创建资源管理器");
        Self {
            resources: HashMap::new(),
            search_paths: Vec::new(),
        }
    }

    pub fn add_search_path(&mut self, path: &str) {
        log::info!("ResourceManager::add_search_path: 添加搜索路径 {}", path);
        self.search_paths.push(path.to_string());
    }

    pub fn load_file(&mut self, filename: &str) -> Option<Vec<u8>> {
        log::info!("ResourceManager::load_file: 加载文件 {}", filename);
        if let Some(data) = self.resources.get(filename) {
            log::debug!("ResourceManager::load_file: 文件 {} 已缓存，大小 {} 字节", filename, data.len());
            return Some(data.clone());
        }
        for path in &self.search_paths {
            let full_path = format!("{}/{}", path, filename);
            log::trace!("ResourceManager::load_file: 尝试路径 {}", full_path);
            if let Ok(data) = fs::read(&full_path) {
                log::info!("ResourceManager::load_file: 文件 {} 加载成功，大小 {} 字节", filename, data.len());
                self.resources.insert(filename.to_string(), data.clone());
                return Some(data);
            }
        }
        log::warn!("ResourceManager::load_file: 文件 {} 未找到", filename);
        None
    }

    pub fn read_string(&mut self, filename: &str) -> Option<String> {
        log::info!("ResourceManager::read_string: 读取字符串文件 {}", filename);
        let data = self.load_file(filename)?;
        String::from_utf8(data).ok()
    }
}
