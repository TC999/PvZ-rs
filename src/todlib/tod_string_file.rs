// 对应 C++ 中的 TodStringFile.h / TodStringFile.cpp
// 多语言字符串文件管理

use std::collections::HashMap;
use std::fs;
use encoding_rs::UTF_8;

pub struct TodStringFile {
    strings: HashMap<String, String>,
}

impl TodStringFile {
    pub fn new() -> Self {
        log::debug!("TodStringFile::new: 创建字符串文件管理器");
        Self {
            strings: HashMap::new(),
        }
    }

    pub fn load_file(&mut self, filename: &str) -> bool {
        log::info!("TodStringFile::load_file: 加载字符串文件 {}", filename);
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                log::error!("TodStringFile::load_file: 文件 {} 读取失败: {}", filename, e);
                return false;
            }
        };
        log::debug!("TodStringFile::load_file: 文件 {} 读取成功，大小 {} 字节", filename, content.len());
        self.parse_content(&content);
        log::info!("TodStringFile::load_file: 字符串文件 {} 加载完成，共 {} 个字符串", filename, self.strings.len());
        true
    }

    fn parse_content(&mut self, content: &str) {
        log::trace!("TodStringFile::parse_content: 解析字符串内容");
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            if let Some(idx) = line.find('=') {
                let key = line[..idx].trim();
                let value = line[idx + 1..].trim();
                self.strings.insert(key.to_string(), value.to_string());
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        log::trace!("TodStringFile::get: 获取字符串 {}", key);
        self.strings.get(key).map(|s| s.as_str())
    }

    pub fn get_or_default<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        log::trace!("TodStringFile::get_or_default: 获取字符串 {}，默认值 {}", key, default);
        self.strings.get(key).map(|s| s.as_str()).unwrap_or(default)
    }
}
