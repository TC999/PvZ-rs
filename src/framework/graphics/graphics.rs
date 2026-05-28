// 对应 C++ 中的 SexyAppFramework/graphics/Graphics.h
// 图形绘制上下文 - 封装 SDL2 Canvas 渲染

use super::color::Color;
use super::font::Font;
use super::image::Image;
use crate::framework::misc::rect::TRect;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect as SdlRect;

pub struct Graphics {
    pub width: i32,
    pub height: i32,
}

impl Graphics {
    pub fn new(width: i32, height: i32) -> Self {
        log::debug!("Graphics::new: 创建图形上下文，尺寸 {}x{}", width, height);
        Self { width, height }
    }

    /// 填充矩形
    pub fn fill_rect(&self, canvas: &mut Canvas<Window>, rect: &TRect, color: &Color) {
        log::trace!("Graphics::fill_rect: 填充矩形 ({}, {}, {}, {})，颜色 ({}, {}, {}, {})", 
            rect.x, rect.y, rect.width, rect.height, color.r, color.g, color.b, color.a);
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a));
        let r = SdlRect::new(rect.x, rect.y, rect.width as u32, rect.height as u32);
        let _ = canvas.fill_rect(r);
    }

    /// 绘制图像
    pub fn draw_image(&self, canvas: &mut Canvas<Window>, image: &Image, x: i32, y: i32) {
        log::trace!("Graphics::draw_image: 绘制图像，位置 ({}, {}), 尺寸 {}x{}", x, y, image.width, image.height);
        // FIXME: 需要将 Image 数据创建为 SDL2 Texture 才能绘制
        // 暂时绘制占位矩形
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 255));
        let r = SdlRect::new(x, y, image.width as u32, image.height as u32);
        let _ = canvas.draw_rect(r);
    }

    /// 绘制带缩放的图像
    pub fn draw_image_rect(
        &self,
        canvas: &mut Canvas<Window>,
        image: &Image,
        dest_x: i32,
        dest_y: i32,
        dest_w: i32,
        dest_h: i32,
    ) {
        log::trace!("Graphics::draw_image_rect: 绘制缩放图像，目标 ({}, {}, {}, {}), 源尺寸 {}x{}", 
            dest_x, dest_y, dest_w, dest_h, image.width, image.height);
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 255));
        let r = SdlRect::new(dest_x, dest_y, dest_w as u32, dest_h as u32);
        let _ = canvas.draw_rect(r);
    }

    /// 绘制线段
    pub fn draw_line(&self, canvas: &mut Canvas<Window>, x1: i32, y1: i32, x2: i32, y2: i32, color: &Color) {
        log::trace!("Graphics::draw_line: 绘制线段 ({}, {}) -> ({}, {}), 颜色 ({}, {}, {}, {})", 
            x1, y1, x2, y2, color.r, color.g, color.b, color.a);
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a));
        let _ = canvas.draw_line((x1, y1), (x2, y2));
    }

    /// 绘制圆形
    pub fn fill_circle(&self, _canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32, color: &Color) {
        log::trace!("Graphics::fill_circle: 绘制圆形，中心 ({}, {}), 半径 {}, 颜色 ({}, {}, {}, {})", 
            cx, cy, radius, color.r, color.g, color.b, color.a);
        // FIXME: SDL2 Canvas 没有直接画圆的 API，需要用三角形扇实现
    }

    /// 绘制椭圆
    pub fn fill_ellipse(&self, _canvas: &mut Canvas<Window>, rect: &TRect, color: &Color) {
        log::trace!("Graphics::fill_ellipse: 绘制椭圆，区域 ({}, {}, {}, {}), 颜色 ({}, {}, {}, {})", 
            rect.x, rect.y, rect.width, rect.height, color.r, color.g, color.b, color.a);
        // FIXME: 未实现
    }

    /// 绘制字符串
    pub fn draw_string(&self, _canvas: &mut Canvas<Window>, text: &str, x: i32, y: i32, _font: &Font) {
        log::trace!("Graphics::draw_string: 绘制字符串 '{}'，位置 ({}, {})", text, x, y);
        // FIXME: 字体渲染需要 SDL2_ttf 纹理
    }

    /// 设置裁剪区域
    pub fn set_clip_rect(&self, canvas: &mut Canvas<Window>, rect: &TRect) {
        log::trace!("Graphics::set_clip_rect: 设置裁剪区域 ({}, {}, {}, {})", rect.x, rect.y, rect.width, rect.height);
        let r = SdlRect::new(rect.x, rect.y, rect.width as u32, rect.height as u32);
        let _ = canvas.set_clip_rect(r);
    }

    /// 清除裁剪
    pub fn clear_clip_rect(&self, canvas: &mut Canvas<Window>) {
        log::trace!("Graphics::clear_clip_rect: 清除裁剪区域");
        canvas.set_clip_rect(None::<SdlRect>);
    }

    /// 设置颜色化
    pub fn set_colorize_images(&self, _canvas: &mut Canvas<Window>, enabled: bool, r: u8, g: u8, b: u8, a: u8) {
        log::trace!("Graphics::set_colorize_images: 设置颜色化，启用 {}，颜色 ({}, {}, {}, {})", enabled, r, g, b, a);
        // FIXME: 通过 SDL2 texture color mod 实现颜色化
    }
}
