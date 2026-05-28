// 对应 C++ 中的 PropertiesParser.h / PropertiesParser.cpp
// 属性文件解析器

use std::collections::HashMap;
use std::fs;

pub struct PropertiesParser {
    props: HashMap<String, String>,
}

impl PropertiesParser {
    pub fn new() -> Self {
        Self { props: HashMap::new() }
    }

    pub fn load(&mut self, filename: &str) -> bool {
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => return false,
        };
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") { continue; }
            if let Some(idx) = line.find('=') {
                let key = line[..idx].trim().to_string();
                let value = line[idx + 1..].trim().to_string();
                self.props.insert(key, value);
            }
        }
        true
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.props.get(key).map(|s| s.as_str())
    }

    pub fn get_int(&self, key: &str) -> Option<i32> {
        self.props.get(key).and_then(|s| s.parse().ok())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.props.get(key).map(|s| s == "true" || s == "1" || s == "yes")
    }
}
