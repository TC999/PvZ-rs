// 对应 C++ 中的 EditWidget.h / EditWidget.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::graphics::Graphics;

pub struct EditWidget {
    pub widget: Widget,
    pub text: String,
    pub max_chars: usize,
    pub cursor_pos: usize,
    pub has_focus: bool,
    pub password_mode: bool,
}

impl EditWidget {
    pub fn new(max_chars: usize) -> Self {
        log::debug!("EditWidget::new: 创建编辑控件，最大字符数 {}", max_chars);
        Self {
            widget: Widget::new(0, 0, 200, 30),
            text: String::new(),
            max_chars,
            cursor_pos: 0,
            has_focus: false,
            password_mode: false,
        }
    }

    pub fn get_text(&self) -> &str {
        log::trace!("EditWidget::get_text: 获取文本 '{}'", self.text);
        &self.text
    }
    pub fn set_text(&mut self, text: &str) {
        log::debug!("EditWidget::set_text: 设置文本 '{}'", text);
        self.text = text.to_string();
    }
}

impl WidgetLike for EditWidget {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }
    fn draw(&self, _g: &Graphics) {
        if !self.widget.visible { return; }
        // TODO: 绘制文本框
    }
}
