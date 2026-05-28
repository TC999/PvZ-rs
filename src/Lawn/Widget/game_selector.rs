// 对应 C++ 中的 GameSelector.h / GameSelector.cpp
pub struct GameSelector {
    pub active: bool,
    pub selected_page: i32,
    pub selected_level: i32,
}
impl GameSelector {
    pub fn new() -> Self {
        log::debug!("GameSelector::new: 创建游戏选择器");
        Self { active: false, selected_page: 0, selected_level: 0 }
    }
    pub fn update(&mut self) {
        log::trace!("GameSelector::update: 更新游戏选择器，active={}", self.active);
    }
}
