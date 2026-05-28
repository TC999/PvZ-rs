// 对应 C++ 中的 Sexy.TodLib/TodCommon.h 和 TodCommon.cpp
// 通用工具函数

pub struct SexyString(String);

impl SexyString {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn empty() -> Self {
        Self(String::new())
    }

    pub fn c_str(&self) -> &str {
        &self.0
    }
}

// 字符串转整数，对应 C++ 中的 atoi 或 std::stoi
pub fn string_to_int(s: &str) -> i32 {
    log::trace!("tod_common::string_to_int: 字符串 '{}' 转整数", s);
    let result = s.parse().unwrap_or(0);
    log::trace!("tod_common::string_to_int: 结果 {}", result);
    result
}

// 字符串转浮点数
pub fn string_to_float(s: &str) -> f64 {
    log::trace!("tod_common::string_to_float: 字符串 '{}' 转浮点数", s);
    let result = s.parse().unwrap_or(0.0);
    log::trace!("tod_common::string_to_float: 结果 {}", result);
    result
}

// 整数范围检查
pub fn int_range_check(val: i32, min: i32, max: i32) -> i32 {
    log::trace!("tod_common::int_range_check: 检查值 {} 范围 [{}, {}]", val, min, max);
    let result = if val < min { min } else if val > max { max } else { val };
    log::trace!("tod_common::int_range_check: 结果 {}", result);
    result
}
