// 对应 C++ 中的 ReanimAtlas.h / ReanimAtlas.cpp
// 动画图集 - 管理动画帧和精灵图集

pub struct ReanimAtlasImage {
    pub filename: String,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

pub struct ReanimAtlas {
    pub images: Vec<ReanimAtlasImage>,
}

impl ReanimAtlas {
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
        }
    }

    pub fn add_image(&mut self, filename: &str, data: Vec<u8>, width: i32, height: i32) {
        self.images.push(ReanimAtlasImage {
            filename: filename.to_string(),
            width,
            height,
            data,
        });
    }

    pub fn get_image(&self, index: usize) -> Option<&ReanimAtlasImage> {
        self.images.get(index)
    }
}
