// 对应 C++ 中的 UserDialog.h / UserDialog.cpp
pub struct UserDialog {
    pub active: bool,
    pub users: Vec<String>,
    pub selected_index: i32,
    pub result: i32,
}
impl UserDialog {
    pub fn new() -> Self {
        log::debug!("UserDialog::new: 创建用户对话框");
        Self { active: false, users: Vec::new(), selected_index: -1, result: 0 }
    }
    pub fn update(&mut self) {
        log::trace!("UserDialog::update: 更新用户对话框，active={}", self.active);
    }
}
