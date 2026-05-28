// 对应 C++ 中的 CreditScreen.h / CreditScreen.cpp
pub struct CreditScreen {
    pub active: bool,
    pub state: i32,
    pub y_offset: i32,
    pub counter: i32,
}
impl CreditScreen {
    pub fn new() -> Self {
        log::debug!("CreditScreen::new: 创建制作人员名单屏幕");
        Self { active: false, state: 0, y_offset: 600, counter: 0 }
    }
    pub fn update(&mut self) {
        log::trace!("CreditScreen::update: 更新制作人员名单屏幕，active={}, counter={}", self.active, self.counter);
        if self.active { self.counter += 1; }
    }
}
