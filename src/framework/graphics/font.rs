// 对应 C++ 中的 SexyAppFramework/graphics/Font.h
// 字体

pub struct Font {
    pub name: String,
    pub size: i32,
}

impl Font {
    pub fn new(name: &str, size: i32) -> Self {
        Self { name: name.to_string(), size }
    }
}
