// 对应 C++ 中的 MessageWidget.h / MessageWidget.cpp

use crate::const_enums::*;

pub struct MessageWidget {
    pub text: String,
    pub message_style: MessageStyle,
    pub x: i32,
    pub y: i32,
    pub visible: bool,
    pub duration: i32,
    pub elapsed: i32,
    pub fade_in: bool,
    pub fade_out: bool,
    pub alpha: f32,
}

impl MessageWidget {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            message_style: MessageStyle::Off,
            x: 0,
            y: 0,
            visible: false,
            duration: 300,
            elapsed: 0,
            fade_in: true,
            fade_out: true,
            alpha: 0.0,
        }
    }

    pub fn show_message(&mut self, text: &str, style: MessageStyle) {
        self.text = text.to_string();
        self.message_style = style;
        self.visible = true;
        self.elapsed = 0;
        self.alpha = 0.0;
    }

    pub fn update(&mut self) {
        if !self.visible { return; }
        self.elapsed += 1;
        if self.elapsed >= self.duration {
            self.visible = false;
        }
    }
}
