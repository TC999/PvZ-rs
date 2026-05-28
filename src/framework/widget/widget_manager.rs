// 对应 C++ 中的 WidgetManager.h / WidgetManager.cpp
// UI 控件管理器

use super::widget::Widget;
use crate::framework::graphics::graphics::Graphics;

pub struct WidgetManager {
    pub widgets: Vec<Box<dyn WidgetLike>>,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub focus_widget: Option<usize>,
    pub down_widget: Option<usize>,
    pub over_widget: Option<usize>,
    pub width: i32,
    pub height: i32,
    pub has_focus: bool,
    pub update_count: i32,
}

pub trait WidgetLike {
    fn as_widget(&self) -> &Widget;
    fn as_widget_mut(&mut self) -> &mut Widget;
    fn update(&mut self) {}
    fn draw(&self, _g: &Graphics) {}
    fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {}
    fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) {}
    fn mouse_move(&mut self, _x: i32, _y: i32) {}
    fn mouse_enter(&mut self) {}
    fn mouse_leave(&mut self) {}
    fn key_down(&mut self, _key: i32) {}
    fn key_up(&mut self, _key: i32) {}
    fn char_input(&mut self, _ch: char) {}
}

impl WidgetManager {
    pub fn new() -> Self {
        log::debug!("WidgetManager::new: 创建控件管理器");
        Self {
            widgets: Vec::new(),
            mouse_x: 0,
            mouse_y: 0,
            focus_widget: None,
            down_widget: None,
            over_widget: None,
            width: 0,
            height: 0,
            has_focus: false,
            update_count: 0,
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn WidgetLike>) -> usize {
        let idx = self.widgets.len();
        self.widgets.push(widget);
        idx
    }

    pub fn get_widget(&self, index: usize) -> Option<&dyn WidgetLike> {
        self.widgets.get(index).map(|w| w.as_ref())
    }

    // pub fn get_widget_mut 需要更复杂的生命周期管理，
    // 使用 dispatch_* 方法代替直接返回可变引用
    pub fn with_widget<R>(&mut self, index: usize, f: impl FnOnce(&mut dyn WidgetLike) -> R) -> Option<R> {
        self.widgets.get_mut(index).map(|w| f(w.as_mut()))
    }

    pub fn update(&mut self) {
        log::trace!("WidgetManager::update: 更新 {} 个控件，更新次数 {}", self.widgets.len(), self.update_count);
        for i in 0..self.widgets.len() {
            self.widgets[i].update();
        }
        self.update_count += 1;
    }

    pub fn draw(&self, g: &Graphics) {
        log::trace!("WidgetManager::draw: 绘制 {} 个控件", self.widgets.len());
        for w in &self.widgets {
            if w.as_widget().visible {
                w.draw(g);
            }
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn mouse_down(&mut self, x: i32, y: i32, click_count: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn mouse_up(&mut self, x: i32, y: i32, click_count: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn key_down(&mut self, _key: i32) {}

    pub fn dispose_widgets(&mut self) {
        self.widgets.clear();
    }
}
