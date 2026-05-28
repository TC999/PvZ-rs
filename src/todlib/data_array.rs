// 对应 C++ 中的 DataArray.h
// 数据数组 - 用于从定义文件中读取数组数据

use std::collections::HashMap;

pub struct DataArray {
    data: Vec<String>,
    key_values: HashMap<String, String>,
}

impl DataArray {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            key_values: HashMap::new(),
        }
    }

    pub fn push(&mut self, val: String) {
        self.data.push(val);
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.data.get(index)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get_string(&self, index: usize) -> Option<&str> {
        self.data.get(index).map(|s| s.as_str())
    }

    pub fn get_int(&self, index: usize) -> Option<i32> {
        self.data.get(index).and_then(|s| s.parse().ok())
    }

    pub fn get_float(&self, index: usize) -> Option<f64> {
        self.data.get(index).and_then(|s| s.parse().ok())
    }

    pub fn get_bool(&self, index: usize) -> Option<bool> {
        self.data.get(index).map(|s| s == "true" || s == "1")
    }

    pub fn set_key_value(&mut self, key: &str, val: &str) {
        self.key_values.insert(key.to_string(), val.to_string());
    }

    pub fn get_key_value(&self, key: &str) -> Option<&str> {
        self.key_values.get(key).map(|s| s.as_str())
    }
}
