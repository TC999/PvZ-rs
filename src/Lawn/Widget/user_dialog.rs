// 对应 C++ 中的 UserDialog.h / UserDialog.cpp
pub struct UserDialog {
    pub active: bool,
    pub users: Vec<String>,
    pub selected_index: i32,
    pub result: i32,
}
impl UserDialog {
    pub fn new() -> Self { Self { active: false, users: Vec::new(), selected_index: -1, result: 0 } }
    pub fn update(&mut self) {}
}
