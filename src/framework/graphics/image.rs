// 对应 C++ 中的 SexyAppFramework/graphics/Image.h
// 图像

use crate::framework::misc::rect::TRect;

pub struct Image {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height * 4) as usize;
        Self {
            width,
            height,
            data: vec![0u8; size],
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Option<(u8, u8, u8, u8)> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        Some((
            self.data[idx],
            self.data[idx + 1],
            self.data[idx + 2],
            self.data[idx + 3],
        ))
    }
}
