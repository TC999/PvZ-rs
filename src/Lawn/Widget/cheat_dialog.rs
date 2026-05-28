// 对应 C++ 中的 CheatDialog.h / CheatDialog.cpp
pub struct CheatDialog {
    pub active: bool,
    pub input: String,
    pub error: bool,
}
impl CheatDialog {
    pub fn new() -> Self { Self { active: false, input: String::new(), error: false } }
    pub fn update(&mut self) {}
}
