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
        Self {
            resources: HashMap::new(),
            search_paths: Vec::new(),
        }
    }

    pub fn add_search_path(&mut self, path: &str) {
        self.search_paths.push(path.to_string());
    }

    pub fn load_file(&mut self, filename: &str) -> Option<Vec<u8>> {
        if let Some(data) = self.resources.get(filename) {
            return Some(data.clone());
        }
        for path in &self.search_paths {
            let full_path = format!("{}/{}", path, filename);
            if let Ok(data) = fs::read(&full_path) {
                self.resources.insert(filename.to_string(), data.clone());
                return Some(data);
            }
        }
        None
    }

    pub fn read_string(&mut self, filename: &str) -> Option<String> {
        let data = self.load_file(filename)?;
        String::from_utf8(data).ok()
    }
}
