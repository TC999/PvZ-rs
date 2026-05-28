// 对应 C++ 中的 Definition.h / Definition.cpp
// 定义系统 - 从 XML / 配置文件加载游戏数据定义

use crate::todlib::data_array::DataArray;
use std::collections::HashMap;

pub struct Definition {
    pub name: String,
    pub fields: HashMap<String, String>,
    pub sub_definitions: Vec<Definition>,
}

impl Definition {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: HashMap::new(),
            sub_definitions: Vec::new(),
        }
    }

    pub fn from_data_array(da: &DataArray) -> Option<Self> {
        if da.is_empty() {
            return None;
        }
        let name = da.get_string(0)?.to_string();
        Some(Self {
            name,
            fields: HashMap::new(),
            sub_definitions: Vec::new(),
        })
    }
}

// 定义文件解析器
pub struct DefinitionParser {
    definitions: Vec<Definition>,
}

impl DefinitionParser {
    pub fn new() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }

    pub fn parse_file(&mut self, _filename: &str) {
        // TODO: 实现 XML 解析
    }

    pub fn get_definition(&self, name: &str) -> Option<&Definition> {
        self.definitions.iter().find(|d| d.name == name)
    }
}
