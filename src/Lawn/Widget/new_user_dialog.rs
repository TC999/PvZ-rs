// 对应 C++ 中的 NewUserDialog.h / NewUserDialog.cpp
pub struct NewUserDialog {
    pub active: bool,
    pub username: String,
    pub result: i32,
}
impl NewUserDialog {
    pub fn new() -> Self { Self { active: false, username: String::new(), result: 0 } }
    pub fn update(&mut self) {}
}
