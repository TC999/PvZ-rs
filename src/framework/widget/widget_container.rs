// 对应 C++ 中的 WidgetContainer.h / WidgetContainer.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::graphics::Graphics;

pub struct WidgetContainer {
    pub widget: Widget,
    pub children: Vec<Box<dyn WidgetLike>>,
}

impl WidgetContainer {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        log::debug!("WidgetContainer::new: 创建控件容器，位置 ({}, {}), 尺寸 {}x{}", x, y, width, height);
        Self {
            widget: Widget::new(x, y, width, height),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Box<dyn WidgetLike>) -> usize {
        let idx = self.children.len();
        log::debug!("WidgetContainer::add_child: 添加子控件，索引 {}", idx);
        self.children.push(child);
        idx
    }

    pub fn remove_all(&mut self) {
        log::info!("WidgetContainer::remove_all: 移除所有子控件");
        self.children.clear();
    }

    pub fn get_child(&self, index: usize) -> Option<&dyn WidgetLike> {
        log::trace!("WidgetContainer::get_child: 获取子控件，索引 {}", index);
        self.children.get(index).map(|w| w.as_ref())
    }
}

impl WidgetLike for WidgetContainer {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }

    fn update(&mut self) {
        for i in 0..self.children.len() {
            self.children[i].update();
        }
    }

    fn draw(&self, g: &Graphics) {
        if !self.widget.visible { return; }
        for child in &self.children {
            if child.as_widget().visible {
                child.draw(g);
            }
        }
    }
}
