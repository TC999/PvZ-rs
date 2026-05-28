// 对应 C++ 中的 PropertiesParser.h / PropertiesParser.cpp
// 属性文件解析器

use std::collections::HashMap;
use std::fs;

pub struct PropertiesParser {
    props: HashMap<String, String>,
}

impl PropertiesParser {
    pub fn new() -> Self {
        log::debug!("PropertiesParser::new: 创建属性解析器");
        Self { props: HashMap::new() }
    }

    pub fn load(&mut self, filename: &str) -> bool {
        log::info!("PropertiesParser::load: 加载属性文件 {}", filename);
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                log::error!("PropertiesParser::load: 文件 {} 读取失败: {}", filename, e);
                return false;
            }
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
        log::info!("PropertiesParser::load: 属性文件 {} 加载完成，共 {} 个属性", filename, self.props.len());
        true
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        log::trace!("PropertiesParser::get_string: 获取字符串 {}", key);
        self.props.get(key).map(|s| s.as_str())
    }

    pub fn get_int(&self, key: &str) -> Option<i32> {
        log::trace!("PropertiesParser::get_int: 获取整数 {}", key);
        self.props.get(key).and_then(|s| s.parse().ok())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        log::trace!("PropertiesParser::get_bool: 获取布尔值 {}", key);
        self.props.get(key).map(|s| s == "true" || s == "1" || s == "yes")
    }
}
