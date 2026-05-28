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
        log::debug!("DescParser::new: 创建描述文件解析器");
        Self { entries: Vec::new() }
    }

    pub fn parse(&mut self, content: &str) {
        log::info!("DescParser::parse: 解析描述文件内容");
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
                log::trace!("DescParser::parse: 解析条目 {}，值 {:?}", key, values);
                self.entries.push(DescEntry { key, values });
            }
        }
        log::info!("DescParser::parse: 解析完成，共 {} 个条目", self.entries.len());
    }

    pub fn get_entry(&self, key: &str) -> Option<&DescEntry> {
        log::trace!("DescParser::get_entry: 获取条目 {}", key);
        self.entries.iter().find(|e| e.key == key)
    }
}
