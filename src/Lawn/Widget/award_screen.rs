// 对应 C++ 中的 AwardScreen.h / AwardScreen.cpp
use crate::const_enums::*;
pub struct AwardScreen {
    pub active: bool,
    pub award_type: AwardType,
    pub counter: i32,
}
impl AwardScreen {
    pub fn new() -> Self {
        log::debug!("AwardScreen::new: 创建奖励屏幕");
        Self { active: false, award_type: AwardType::ForLevel, counter: 0 }
    }
    pub fn update(&mut self) {
        log::trace!("AwardScreen::update: 更新奖励屏幕，active={}, counter={}", self.active, self.counter);
        if self.active { self.counter += 1; }
    }
}
