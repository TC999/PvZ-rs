// 对应 C++ 中的 Widget.h / Widget.cpp
// 基础 UI 控件

use crate::framework::graphics::color::Color;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::misc::rect::TRect;

pub struct Widget {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
    pub enabled: bool,
    pub has_focus: bool,
    pub has_transparencies: bool,
    pub m_do_not_resize: bool,
    pub m_parent: Option<usize>,
    pub m_id: usize,
    pub m_disabled_count: i32,
    pub m_colors: Vec<Color>,
}

impl Widget {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        log::debug!("Widget::new: 创建控件，位置 ({}, {}), 尺寸 {}x{}", x, y, width, height);
        Self {
            x, y, width, height,
            visible: true,
            enabled: true,
            has_focus: false,
            has_transparencies: false,
            m_do_not_resize: false,
            m_parent: None,
            m_id: 0,
            m_disabled_count: 0,
            m_colors: Vec::new(),
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        log::trace!("Widget::set_visible: 设置可见性 {}", visible);
        self.visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        log::trace!("Widget::set_enabled: 设置启用状态 {}", enabled);
        self.enabled = enabled;
    }

    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        log::trace!("Widget::resize: 调整控件，位置 ({}, {}), 尺寸 {}x{}", x, y, w, h);
        self.x = x;
        self.y = y;
        self.width = w;
        self.height = h;
    }

    pub fn get_rect(&self) -> TRect {
        TRect::new(self.x, self.y, self.width, self.height)
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        let result = self.get_rect().contains(px, py);
        log::trace!("Widget::contains: 检查点 ({}, {}) 是否在控件内，结果 {}", px, py, result);
        result
    }

    pub fn update(&mut self) {
        log::trace!("Widget::update: 更新控件");
        // 基类更新逻辑
    }

    pub fn draw(&self, _g: &Graphics) {
        log::trace!("Widget::draw: 绘制控件");
        // 基类绘制逻辑
    }

    pub fn mouse_down(&mut self, x: i32, y: i32, click_count: i32) {
        log::debug!("Widget::mouse_down: 鼠标按下 ({}, {}), 点击次数 {}", x, y, click_count);
    }
    pub fn mouse_up(&mut self, x: i32, y: i32, click_count: i32) {
        log::debug!("Widget::mouse_up: 鼠标释放 ({}, {}), 点击次数 {}", x, y, click_count);
    }
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        log::trace!("Widget::mouse_move: 鼠标移动 ({}, {})", x, y);
    }
    pub fn mouse_enter(&mut self) {
        log::trace!("Widget::mouse_enter: 鼠标进入控件");
    }
    pub fn mouse_leave(&mut self) {
        log::trace!("Widget::mouse_leave: 鼠标离开控件");
    }
    pub fn key_down(&mut self, key: i32) {
        log::debug!("Widget::key_down: 按键按下 {}", key);
    }
    pub fn key_up(&mut self, key: i32) {
        log::debug!("Widget::key_up: 按键释放 {}", key);
    }
    pub fn char_input(&mut self, ch: char) {
        log::debug!("Widget::char_input: 字符输入 '{}'", ch);
    }
    pub fn got_focus(&mut self) {
        log::trace!("Widget::got_focus: 获得焦点");
    }
    pub fn lost_focus(&mut self) {
        log::trace!("Widget::lost_focus: 失去焦点");
    }
}
