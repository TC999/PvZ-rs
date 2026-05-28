// 对应 C++ 中的 SexyAppFramework/graphics/Graphics.h
// 图形绘制上下文

use super::color::Color;
use super::font::Font;
use super::image::Image;
use crate::framework::misc::rect::TRect;

pub struct Graphics {
    pub width: i32,
    pub height: i32,
}

impl Graphics {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn fill_rect(&self, _rect: &TRect, _color: &Color) {
        // TODO: 实现通过 SDL2/OpenGL 绘制矩形
    }

    pub fn draw_image(&self, _image: &Image, _x: i32, _y: i32) {
        // TODO: 实现图像绘制
    }

    pub fn draw_string(&self, _text: &str, _x: i32, _y: i32, _font: &Font) {
        // TODO: 实现文字绘制
    }
}
