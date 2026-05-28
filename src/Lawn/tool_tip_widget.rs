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
        self.text = text.to_string();
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn show(&mut self) { self.visible = true; }
    pub fn hide(&mut self) { self.visible = false; }
}
