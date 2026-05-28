// 对应 C++ 中的 CheatDialog.h / CheatDialog.cpp
pub struct CheatDialog {
    pub active: bool,
    pub input: String,
    pub error: bool,
}
impl CheatDialog {
    pub fn new() -> Self {
        log::debug!("CheatDialog::new: 创建作弊对话框");
        Self { active: false, input: String::new(), error: false }
    }
    pub fn update(&mut self) {
        log::trace!("CheatDialog::update: 更新作弊对话框，active={}", self.active);
    }
}
