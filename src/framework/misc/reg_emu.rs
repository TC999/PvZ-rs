// 对应 C++ 中的 RegEmu.h / RegEmu.cpp
// 注册表模拟 - 用于存储用户配置（Windows 注册表的跨平台替代）

use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegEmu {
    values: HashMap<String, String>,
    filename: String,
}

impl RegEmu {
    pub fn new(filename: &str) -> Self {
        let mut reg = Self {
            values: HashMap::new(),
            filename: filename.to_string(),
        };
        reg.load();
        reg
    }

    fn load(&mut self) {
        if let Ok(content) = fs::read_to_string(&self.filename) {
            for line in content.lines() {
                if let Some(idx) = line.find('=') {
                    let key = line[..idx].trim().to_string();
                    let value = line[idx + 1..].trim().to_string();
                    self.values.insert(key, value);
                }
            }
        }
    }

    pub fn save(&self) {
        let content: String = self.values.iter()
            .map(|(k, v)| format!("{}={}\n", k, v))
            .collect();
        let _ = fs::write(&self.filename, content);
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.values.get(key).cloned()
    }

    pub fn set_string(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    pub fn get_int(&self, key: &str) -> Option<i32> {
        self.values.get(key).and_then(|v| v.parse().ok())
    }

    pub fn set_int(&mut self, key: &str, value: i32) {
        self.set_string(key, &value.to_string());
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.values.get(key).map(|v| v == "true" || v == "1")
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.set_string(key, if value { "true" } else { "false" });
    }
}
