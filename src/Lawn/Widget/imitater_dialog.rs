// 对应 C++ 中的 ImitaterDialog.h / ImitaterDialog.cpp
use crate::const_enums::*;
pub struct ImitaterDialog {
    pub active: bool,
    pub selected_seed: SeedType,
    pub imitater_seed: SeedType,
}
impl ImitaterDialog {
    pub fn new() -> Self { Self { active: false, selected_seed: SeedType::None, imitater_seed: SeedType::None } }
    pub fn update(&mut self) {}
}
