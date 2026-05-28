// 对应 C++ 中的 Dialog.h / Dialog.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::misc::rect::TRect;

pub struct Dialog {
    pub widget: Widget,
    pub title: String,
    pub result: i32,
    pub modal: bool,
    pub is_closing: bool,
    pub ok_button_id: usize,
    pub cancel_button_id: usize,
    pub m_header_height: i32,
}

impl Dialog {
    pub fn new(title: &str) -> Self {
        log::debug!("Dialog::new: 创建对话框，标题 {}", title);
        Self {
            widget: Widget::new(0, 0, 400, 300),
            title: title.to_string(),
            result: 0,
            modal: true,
            is_closing: false,
            ok_button_id: 0,
            cancel_button_id: 0,
            m_header_height: 30,
        }
    }

    pub fn set_centered(&mut self, screen_width: i32, screen_height: i32) {
        log::trace!("Dialog::set_centered: 设置对话框居中，屏幕尺寸 {}x{}", screen_width, screen_height);
        self.widget.x = (screen_width - self.widget.width) / 2;
        self.widget.y = (screen_height - self.widget.height) / 2;
    }

    pub fn get_result(&self) -> i32 {
        log::trace!("Dialog::get_result: 获取对话框结果 {}", self.result);
        self.result
    }

    pub fn close(&mut self) {
        log::info!("Dialog::close: 关闭对话框");
        self.is_closing = true;
    }
}

impl WidgetLike for Dialog {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }

    fn draw(&self, g: &Graphics) {
        if !self.widget.visible { return; }
        // TODO: 绘制对话框背景和边框
    }

    fn update(&mut self) {
        // TODO: 对话框动画/过渡
    }
}
