// 对应 C++ 中的 AwardScreen.h / AwardScreen.cpp
use crate::const_enums::*;
pub struct AwardScreen {
    pub active: bool,
    pub award_type: AwardType,
    pub counter: i32,
}
impl AwardScreen {
    pub fn new() -> Self { Self { active: false, award_type: AwardType::ForLevel, counter: 0 } }
    pub fn update(&mut self) { if self.active { self.counter += 1; } }
}
