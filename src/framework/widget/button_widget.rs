// 对应 C++ 中的 ButtonWidget.h / ButtonWidget.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::color::Color;
use crate::framework::graphics::graphics::Graphics;

pub struct ButtonWidget {
    pub widget: Widget,
    pub label: String,
    pub is_down: bool,
    pub is_over: bool,
    pub m_colors: Vec<Color>,
    pub m_button_image: Option<usize>,
    pub m_over_image: Option<usize>,
    pub m_down_image: Option<usize>,
    pub m_disabled_image: Option<usize>,
    pub m_track_mouse: bool,
    pub m_do_finger: bool,
}

impl ButtonWidget {
    pub fn new(id: usize, label: &str) -> Self {
        Self {
            widget: Widget::new(0, 0, 100, 30),
            label: label.to_string(),
            is_down: false,
            is_over: false,
            m_colors: Vec::new(),
            m_button_image: None,
            m_over_image: None,
            m_down_image: None,
            m_disabled_image: None,
            m_track_mouse: false,
            m_do_finger: false,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.label = text.to_string();
    }
}

impl WidgetLike for ButtonWidget {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }

    fn draw(&self, g: &Graphics) {
        if !self.widget.visible { return; }
        // TODO: 绘制按钮
    }

    fn mouse_down(&mut self, x: i32, y: i32, click_count: i32) {
        if self.widget.contains(x, y) {
            self.is_down = true;
        }
    }

    fn mouse_up(&mut self, x: i32, y: i32, click_count: i32) {
        if self.is_down && self.widget.contains(x, y) {
            self.is_down = false;
            // TODO: 触发按钮点击回调
        }
        self.is_down = false;
    }

    fn mouse_enter(&mut self) { self.is_over = true; }
    fn mouse_leave(&mut self) { self.is_over = false; }
}
