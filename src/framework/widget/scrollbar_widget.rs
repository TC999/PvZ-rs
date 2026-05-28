// 对应 C++ 中的 ScrollbarWidget.h / ScrollbarWidget.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::graphics::Graphics;

pub struct ScrollbarWidget {
    pub widget: Widget,
    pub value: i32,
    pub max_value: i32,
    pub page_size: i32,
    pub horizontal: bool,
    pub is_dragging: bool,
}

impl ScrollbarWidget {
    pub fn new(horizontal: bool) -> Self {
        log::debug!("ScrollbarWidget::new: 创建滚动条，水平={}", horizontal);
        Self {
            widget: Widget::new(0, 0, if horizontal { 100 } else { 16 }, if horizontal { 16 } else { 100 }),
            value: 0,
            max_value: 100,
            page_size: 10,
            horizontal,
            is_dragging: false,
        }
    }

    pub fn set_max(&mut self, max: i32) {
        log::trace!("ScrollbarWidget::set_max: 设置最大值 {}", max);
        self.max_value = max;
    }
    pub fn set_value(&mut self, val: i32) {
        log::trace!("ScrollbarWidget::set_value: 设置值 {}", val);
        self.value = val.clamp(0, self.max_value);
    }
}

impl WidgetLike for ScrollbarWidget {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }
    fn draw(&self, _g: &Graphics) {
        if !self.widget.visible { return; }
        // TODO: 绘制滚动条
    }
    fn mouse_down(&mut self, _x: i32, _y: i32, _c: i32) { self.is_dragging = true; }
    fn mouse_up(&mut self, _x: i32, _y: i32, _c: i32) { self.is_dragging = false; }
}
