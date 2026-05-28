// 对应 C++ 中的 ImageFont.h / ImageFont.cpp
// 图像字体渲染
//
// ASSUMPTION: C++ ImageFont 基于图像的字形映射表进行字符渲染。
// Rust 版本使用 sdl2::ttf 或 bitmap 字体。

/// CharData - 字符数据 (对应 C++ Sexy::CharData)
pub struct CharData {
    pub width: i32,
    pub height: i32,
    pub offset_x: i32,
    pub offset_y: i32,
}

/// ImageFont - 图像字体 (对应 C++ Sexy::ImageFont)
pub struct ImageFont {
    pub char_data: Vec<CharData>,
    pub char_pos: Vec<[i32; 2]>,
    pub font_height: i32,
    pub ascent: i32,
}

impl ImageFont {
    pub fn new() -> Self {
        Self {
            char_data: Vec::new(),
            char_pos: Vec::new(),
            font_height: 0,
            ascent: 0,
        }
    }

    pub fn string_width(&self, _text: &str) -> i32 {
        // FIXME: Actual CharData-based width calculation
        0
    }
}
