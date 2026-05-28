// 对应 C++ 中的 TextWidget.h / TextWidget.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::color::Color;
use crate::framework::graphics::graphics::Graphics;

pub struct TextWidget {
    pub widget: Widget,
    pub text: String,
    pub font: Option<i32>,
    pub color: Color,
    pub multiline: bool,
}

impl TextWidget {
    pub fn new(text: &str) -> Self {
        log::debug!("TextWidget::new: 创建文本控件，文本 '{}'", text);
        Self {
            widget: Widget::new(0, 0, 200, 20),
            text: text.to_string(),
            font: None,
            color: Color::WHITE,
            multiline: false,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        log::debug!("TextWidget::set_text: 设置文本 '{}'", text);
        self.text = text.to_string();
    }
    pub fn get_text(&self) -> &str {
        log::trace!("TextWidget::get_text: 获取文本 '{}'", self.text);
        &self.text
    }
}

impl WidgetLike for TextWidget {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }
    fn draw(&self, _g: &Graphics) {
        if !self.widget.visible { return; }
        // TODO: 绘制文本
    }
}
