// 对应 C++ 中的 TitleScreen.h / TitleScreen.cpp
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::color::Color;
use crate::framework::misc::rect::TRect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct TitleScreen {
    pub active: bool,
    pub state: i32,
    pub counter: i32,
    pub showing: bool,
}

impl TitleScreen {
    pub fn new() -> Self {
        Self { active: false, state: 0, counter: 0, showing: false }
    }
    pub fn update(&mut self) { if self.active { self.counter += 1; } }
    pub fn draw(&self, g: &Graphics, canvas: &mut Canvas<Window>) {
        // 测试渲染：绘制一个简单的绿色背景和文字区域
        if !self.active {
            return;
        }
        // 绘制背景（暗绿色）
        g.fill_rect(canvas, &TRect::new(0, 0, g.width, g.height), &Color::from_rgb(20, 80, 20));
        // 绘制标题区域
        g.fill_rect(canvas, &TRect::new(200, 100, 400, 60), &Color::from_rgb(0, 150, 0));
    }
    pub fn start(&mut self) { self.active = true; self.state = 0; }

    pub fn key_down(&mut self, _key: i32) {}
    pub fn key_up(&mut self, _key: i32) {}
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _clicks: i32) {}
    pub fn mouse_up(&mut self, _x: i32, _y: i32, _clicks: i32) {}
    pub fn mouse_move(&mut self, _x: i32, _y: i32) {}
}
