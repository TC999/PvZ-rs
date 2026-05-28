// 对应 C++ 中的 GameButton.h / GameButton.cpp
pub struct GameButton {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub label: String,
    pub visible: bool,
    pub enabled: bool,
    pub is_down: bool,
    pub is_over: bool,
}
impl GameButton {
    pub fn new(label: &str) -> Self {
        Self { x:0, y:0, width:100, height:30, label:label.to_string(), visible:true, enabled:true, is_down:false, is_over:false }
    }
    pub fn update(&mut self) {}
}
