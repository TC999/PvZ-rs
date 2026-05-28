// 对应 C++ 中的 DescParser.h / DescParser.cpp
// 描述文件解析器

use std::fs;

pub struct DescEntry {
    pub key: String,
    pub values: Vec<String>,
}

pub struct DescParser {
    entries: Vec<DescEntry>,
}

impl DescParser {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn parse(&mut self, content: &str) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") { continue; }
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let values: Vec<String> = parts[1]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                self.entries.push(DescEntry { key, values });
            }
        }
    }

    pub fn get_entry(&self, key: &str) -> Option<&DescEntry> {
        self.entries.iter().find(|e| e.key == key)
    }
}
