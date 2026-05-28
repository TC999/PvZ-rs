// 对应 C++ 中的 GameSelector.h / GameSelector.cpp
pub struct GameSelector {
    pub active: bool,
    pub selected_page: i32,
    pub selected_level: i32,
}
impl GameSelector {
    pub fn new() -> Self { Self { active: false, selected_page: 0, selected_level: 0 } }
    pub fn update(&mut self) {}
}
