// 对应 C++ 中的 Insets.h / Insets.cpp
// 边距/内边距

#[derive(Debug, Clone, Copy)]
pub struct Insets {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Insets {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self { left, top, right, bottom }
    }

    pub fn uniform(value: i32) -> Self {
        Self { left: value, top: value, right: value, bottom: value }
    }
}
