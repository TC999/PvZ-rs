// 对应 C++ 中的 DialogButton.h / DialogButton.cpp

use super::button_widget::ButtonWidget;
use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::graphics::Graphics;

pub struct DialogButton {
    pub button: ButtonWidget,
    pub dialog_result: i32,
}

impl DialogButton {
    pub fn new(label: &str, dialog_result: i32) -> Self {
        log::debug!("DialogButton::new: 创建对话框按钮，标签 {}，结果 {}", label, dialog_result);
        Self {
            button: ButtonWidget::new(0, label),
            dialog_result,
        }
    }
}

impl WidgetLike for DialogButton {
    fn as_widget(&self) -> &Widget { self.button.as_widget() }
    fn as_widget_mut(&mut self) -> &mut Widget { self.button.as_widget_mut() }
    fn draw(&self, g: &Graphics) { self.button.draw(g); }
    fn mouse_down(&mut self, x: i32, y: i32, c: i32) { self.button.mouse_down(x, y, c); }
    fn mouse_up(&mut self, x: i32, y: i32, c: i32) { self.button.mouse_up(x, y, c); }
    fn mouse_enter(&mut self) { self.button.mouse_enter(); }
    fn mouse_leave(&mut self) { self.button.mouse_leave(); }
}
