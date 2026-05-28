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
    s.parse().unwrap_or(0)
}

// 字符串转浮点数
pub fn string_to_float(s: &str) -> f64 {
    s.parse().unwrap_or(0.0)
}

// 整数范围检查
pub fn int_range_check(val: i32, min: i32, max: i32) -> i32 {
    if val < min { min } else if val > max { max } else { val }
}
