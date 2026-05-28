// 对应 C++ 中的 AlmanacDialog.h / AlmanacDialog.cpp
use crate::const_enums::*;
pub struct AlmanacDialog {
    pub active: bool,
    pub page: AlmanacPage,
    pub selected_index: i32,
    pub scroll_offset: i32,
}
impl AlmanacDialog {
    pub fn new() -> Self { Self { active: false, page: AlmanacPage::Index, selected_index: 0, scroll_offset: 0 } }
    pub fn update(&mut self) {}
}
