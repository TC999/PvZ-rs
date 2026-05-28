// 对应 C++ 中的 GLImage.h / GLImage.cpp
// OpenGL 纹理图像包装
//
// ASSUMPTION: C++ GLImage 封装了 OpenGL 纹理创建和渲染。
// Rust 版本中 OpenGL 渲染由 sdl2 的 Window/Canvas 等抽象层处理，
// 此模块仅提供接口兼容性。

use crate::framework::graphics::image::Image;

/// GLImage - OpenGL 纹理图像 (对应 C++ Sexy::GLImage)
pub struct GLImage {
    pub image: Option<Image>,
    pub texture_id: u32,
    pub width: i32,
    pub height: i32,
}

impl GLImage {
    pub fn new() -> Self {
        log::debug!("GLImage::new: 创建 OpenGL 纹理图像");
        Self { image: None, texture_id: 0, width: 0, height: 0 }
    }

    pub fn set_image(&mut self, img: Image) {
        log::info!("GLImage::set_image: 设置图像，尺寸 {}x{}", img.width, img.height);
        self.width = img.width;
        self.height = img.height;
        self.image = Some(img);
    }

    pub fn get_width(&self) -> i32 { self.width }
    pub fn get_height(&self) -> i32 { self.height }
}
