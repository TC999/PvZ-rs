// 对应 C++ 中的 DataArray.h
// 数据数组 - 用于从定义文件中读取数组数据

use std::collections::HashMap;

pub struct DataArray {
    data: Vec<String>,
    key_values: HashMap<String, String>,
}

impl DataArray {
    pub fn new() -> Self {
        log::debug!("DataArray::new: 创建数据数组");
        Self {
            data: Vec::new(),
            key_values: HashMap::new(),
        }
    }

    pub fn push(&mut self, val: String) {
        log::trace!("DataArray::push: 添加值 '{}'", val);
        self.data.push(val);
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        log::trace!("DataArray::get: 获取索引 {} 的值", index);
        self.data.get(index)
    }

    pub fn len(&self) -> usize {
        log::trace!("DataArray::len: 获取长度 {}", self.data.len());
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        log::trace!("DataArray::is_empty: 检查是否为空 {}", self.data.is_empty());
        self.data.is_empty()
    }

    pub fn get_string(&self, index: usize) -> Option<&str> {
        log::trace!("DataArray::get_string: 获取索引 {} 的字符串", index);
        self.data.get(index).map(|s| s.as_str())
    }

    pub fn get_int(&self, index: usize) -> Option<i32> {
        log::trace!("DataArray::get_int: 获取索引 {} 的整数", index);
        self.data.get(index).and_then(|s| s.parse().ok())
    }

    pub fn get_float(&self, index: usize) -> Option<f64> {
        log::trace!("DataArray::get_float: 获取索引 {} 的浮点数", index);
        self.data.get(index).and_then(|s| s.parse().ok())
    }

    pub fn get_bool(&self, index: usize) -> Option<bool> {
        log::trace!("DataArray::get_bool: 获取索引 {} 的布尔值", index);
        self.data.get(index).map(|s| s == "true" || s == "1")
    }

    pub fn set_key_value(&mut self, key: &str, val: &str) {
        log::debug!("DataArray::set_key_value: 设置键值对 {}={}", key, val);
        self.key_values.insert(key.to_string(), val.to_string());
    }

    pub fn get_key_value(&self, key: &str) -> Option<&str> {
        log::trace!("DataArray::get_key_value: 获取键 {} 的值", key);
        self.key_values.get(key).map(|s| s.as_str())
    }
}
