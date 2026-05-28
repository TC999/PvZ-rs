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
        log::info!("RegEmu::new: 创建注册表模拟器，文件 {}", filename);
        let mut reg = Self {
            values: HashMap::new(),
            filename: filename.to_string(),
        };
        reg.load();
        reg
    }

    fn load(&mut self) {
        log::debug!("RegEmu::load: 加载注册表数据，文件 {}", self.filename);
        if let Ok(content) = fs::read_to_string(&self.filename) {
            for line in content.lines() {
                if let Some(idx) = line.find('=') {
                    let key = line[..idx].trim().to_string();
                    let value = line[idx + 1..].trim().to_string();
                    self.values.insert(key, value);
                }
            }
            log::debug!("RegEmu::load: 注册表数据加载完成，共 {} 个值", self.values.len());
        } else {
            log::warn!("RegEmu::load: 注册表文件 {} 读取失败", self.filename);
        }
    }

    pub fn save(&self) {
        log::debug!("RegEmu::save: 保存注册表数据，文件 {}", self.filename);
        let content: String = self.values.iter()
            .map(|(k, v)| format!("{}={}\n", k, v))
            .collect();
        let _ = fs::write(&self.filename, content);
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        log::trace!("RegEmu::get_string: 获取字符串 {}", key);
        self.values.get(key).cloned()
    }

    pub fn set_string(&mut self, key: &str, value: &str) {
        log::debug!("RegEmu::set_string: 设置字符串 {}={}", key, value);
        self.values.insert(key.to_string(), value.to_string());
    }

    pub fn get_int(&self, key: &str) -> Option<i32> {
        log::trace!("RegEmu::get_int: 获取整数 {}", key);
        self.values.get(key).and_then(|v| v.parse().ok())
    }

    pub fn set_int(&mut self, key: &str, value: i32) {
        log::debug!("RegEmu::set_int: 设置整数 {}={}", key, value);
        self.set_string(key, &value.to_string());
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        log::trace!("RegEmu::get_bool: 获取布尔值 {}", key);
        self.values.get(key).map(|v| v == "true" || v == "1")
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        log::debug!("RegEmu::set_bool: 设置布尔值 {}={}", key, value);
        self.set_string(key, if value { "true" } else { "false" });
    }
}
