// 对应 C++ 中的 StoreScreen.h / StoreScreen.cpp
use crate::const_enums::*;
pub struct StoreScreen {
    pub active: bool,
    pub page: StorePages,
    pub selected_item: StoreItem,
    pub money: i32,
}
impl StoreScreen {
    pub fn new() -> Self {
        log::debug!("StoreScreen::new: 创建商店屏幕");
        Self { active: false, page: StorePages::SlotUpgrades, selected_item: StoreItem::PlantGatlingPea, money: 0 }
    }
    pub fn update(&mut self) {
        log::trace!("StoreScreen::update: 更新商店屏幕，active={}", self.active);
    }
}
