// 对应 C++ 中的 ImitaterDialog.h / ImitaterDialog.cpp
use crate::const_enums::*;
pub struct ImitaterDialog {
    pub active: bool,
    pub selected_seed: SeedType,
    pub imitater_seed: SeedType,
}
impl ImitaterDialog {
    pub fn new() -> Self {
        log::debug!("ImitaterDialog::new: 创建模仿者对话框");
        Self { active: false, selected_seed: SeedType::None, imitater_seed: SeedType::None }
    }
    pub fn update(&mut self) {
        log::trace!("ImitaterDialog::update: 更新模仿者对话框，active={}", self.active);
    }
}
