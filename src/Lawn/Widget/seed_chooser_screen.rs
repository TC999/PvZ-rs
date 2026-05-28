// 对应 C++ 中的 SeedChooserScreen.h / SeedChooserScreen.cpp
use crate::const_enums::*;
pub struct SeedChooserScreen {
    pub active: bool,
    pub state: SeedChooserState,
    pub chosen_seeds: Vec<SeedType>,
}
impl SeedChooserScreen {
    pub fn new() -> Self {
        log::debug!("SeedChooserScreen::new: 创建种子选择屏幕");
        Self { active: false, state: SeedChooserState::Normal, chosen_seeds: Vec::new() }
    }
    pub fn update(&mut self) {
        log::trace!("SeedChooserScreen::update: 更新种子选择屏幕，active={}", self.active);
    }
}
