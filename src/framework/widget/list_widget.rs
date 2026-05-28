// 对应 C++ 中的 ListWidget.h / ListWidget.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::graphics::Graphics;

pub struct ListWidget {
    pub widget: Widget,
    pub items: Vec<String>,
    pub selected_index: i32,
    pub item_height: i32,
    pub scroll_offset: i32,
    pub hover_index: i32,
}

impl ListWidget {
    pub fn new() -> Self {
        log::debug!("ListWidget::new: 创建列表控件");
        Self {
            widget: Widget::new(0, 0, 200, 200),
            items: Vec::new(),
            selected_index: -1,
            item_height: 24,
            scroll_offset: 0,
            hover_index: -1,
        }
    }

    pub fn add_item(&mut self, item: &str) {
        log::debug!("ListWidget::add_item: 添加项目 '{}'", item);
        self.items.push(item.to_string());
    }

    pub fn clear(&mut self) {
        log::info!("ListWidget::clear: 清空列表");
        self.items.clear();
        self.selected_index = -1;
    }

    pub fn get_selected(&self) -> Option<&str> {
        log::trace!("ListWidget::get_selected: 获取选中项，索引 {}", self.selected_index);
        if self.selected_index >= 0 && (self.selected_index as usize) < self.items.len() {
            Some(&self.items[self.selected_index as usize])
        } else {
            None
        }
    }
}

impl WidgetLike for ListWidget {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }
    fn draw(&self, _g: &Graphics) {
        if !self.widget.visible { return; }
    }
    fn mouse_move(&mut self, x: i32, y: i32) {
        if self.widget.contains(x, y) {
            let rel_y = y - self.widget.y;
            self.hover_index = (rel_y / self.item_height) as i32;
        } else {
            self.hover_index = -1;
        }
    }
    fn mouse_up(&mut self, x: i32, y: i32, _c: i32) {
        if self.widget.contains(x, y) {
            let rel_y = y - self.widget.y;
            let idx = (rel_y / self.item_height) as i32;
            if idx >= 0 && (idx as usize) < self.items.len() {
                self.selected_index = idx;
            }
        }
    }
}
