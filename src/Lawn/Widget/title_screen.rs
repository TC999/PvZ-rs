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
        log::debug!("TitleScreen::new: 创建标题屏幕");
        Self { active: false, state: 0, counter: 0, showing: false }
    }
    pub fn update(&mut self) {
        log::trace!("TitleScreen::update: 更新标题屏幕，状态 active={}, counter={}", self.active, self.counter);
        if self.active { self.counter += 1; }
    }
    pub fn draw(&self, g: &Graphics, canvas: &mut Canvas<Window>) {
        log::trace!("TitleScreen::draw: 绘制标题屏幕，active={}", self.active);
        // 测试渲染：绘制一个简单的绿色背景和文字区域
        if !self.active {
            log::trace!("TitleScreen::draw: 标题屏幕未激活，跳过绘制");
            return;
        }
        // 绘制背景（暗绿色）
        g.fill_rect(canvas, &TRect::new(0, 0, g.width, g.height), &Color::from_rgb(20, 80, 20));
        // 绘制标题区域
        g.fill_rect(canvas, &TRect::new(200, 100, 400, 60), &Color::from_rgb(0, 150, 0));
        log::trace!("TitleScreen::draw: 标题屏幕绘制完成");
    }
    pub fn start(&mut self) {
        log::info!("TitleScreen::start: 启动标题屏幕");
        self.active = true; self.state = 0;
    }

    pub fn key_down(&mut self, key: i32) {
        log::debug!("TitleScreen::key_down: 按键按下 {}", key);
    }
    pub fn key_up(&mut self, key: i32) {
        log::debug!("TitleScreen::key_up: 按键释放 {}", key);
    }
    pub fn mouse_down(&mut self, x: i32, y: i32, clicks: i32) {
        log::debug!("TitleScreen::mouse_down: 鼠标按下 ({}, {}), 点击次数 {}", x, y, clicks);
    }
    pub fn mouse_up(&mut self, x: i32, y: i32, clicks: i32) {
        log::debug!("TitleScreen::mouse_up: 鼠标释放 ({}, {}), 点击次数 {}", x, y, clicks);
    }
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        log::trace!("TitleScreen::mouse_move: 鼠标移动 ({}, {})", x, y);
    }
}
