// 对应 C++ 中的 SexyAppFramework/graphics/Font.h
// 字体

pub struct Font {
    pub name: String,
    pub size: i32,
}

impl Font {
    pub fn new(name: &str, size: i32) -> Self {
        log::debug!("Font::new: 创建字体，名称 {}，大小 {}", name, size);
        Self { name: name.to_string(), size }
    }
}
