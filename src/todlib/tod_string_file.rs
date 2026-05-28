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
        Self {
            strings: HashMap::new(),
        }
    }

    pub fn load_file(&mut self, filename: &str) -> bool {
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => return false,
        };
        self.parse_content(&content);
        true
    }

    fn parse_content(&mut self, content: &str) {
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
        self.strings.get(key).map(|s| s.as_str())
    }

    pub fn get_or_default<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.strings.get(key).map(|s| s.as_str()).unwrap_or(default)
    }
}
