// 对应 C++ 中的 ToolTipWidget.h / ToolTipWidget.cpp

pub struct ToolTipWidget {
    pub text: String,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub visible: bool,
    pub warning: bool,
    pub center: bool,
}

impl ToolTipWidget {
    pub fn new() -> Self {
        log::debug!("ToolTipWidget::new: 创建工具提示控件");
        Self {
            text: String::new(),
            title: String::new(),
            x: 0,
            y: 0,
            visible: false,
            warning: false,
            center: false,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        log::trace!("ToolTipWidget::set_text: 设置工具提示文本 {}", text);
        self.text = text.to_string();
    }

    pub fn set_title(&mut self, title: &str) {
        log::trace!("ToolTipWidget::set_title: 设置工具提示标题 {}", title);
        self.title = title.to_string();
    }

    pub fn show(&mut self) {
        log::trace!("ToolTipWidget::show: 显示工具提示");
        self.visible = true;
    }
    pub fn hide(&mut self) {
        log::trace!("ToolTipWidget::hide: 隐藏工具提示");
        self.visible = false;
    }
}
