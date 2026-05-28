// 对应 C++ 中的 ChallengeScreen.h / ChallengeScreen.cpp
use crate::const_enums::*;
pub struct ChallengeScreen {
    pub active: bool,
    pub page: ChallengePage,
    pub selected_challenge: i32,
}
impl ChallengeScreen {
    pub fn new() -> Self { Self { active: false, page: ChallengePage::Survival, selected_challenge: 0 } }
    pub fn update(&mut self) {}
}
