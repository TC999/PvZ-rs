// 对应 C++ 中的 LawnDialog.h / LawnDialog.cpp
use crate::const_enums::*;
pub struct LawnDialog {
    pub active: bool,
    pub dialog_type: Dialogs,
    pub result: i32,
    pub message: String,
}
impl LawnDialog {
    pub fn new(dialog_type: Dialogs) -> Self {
        log::debug!("LawnDialog::new: 创建草坪对话框，类型 {:?}", dialog_type);
        Self { active: false, dialog_type, result: 0, message: String::new() }
    }
    pub fn update(&mut self) {
        log::trace!("LawnDialog::update: 更新草坪对话框，active={}", self.active);
    }
}
