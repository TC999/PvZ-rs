// 对应 C++ 中的 GridItem.h / GridItem.cpp

use crate::const_enums::*;

pub struct GridItem {
    pub grid_item_type: GridItemType,
    pub grid_item_state: GridItemState,
    pub x: i32,
    pub y: i32,
    pub col: i32,
    pub row: i32,
    pub active: bool,
    pub counter: i32,
    pub anim_counter: i32,
}

impl GridItem {
    pub fn new(item_type: GridItemType, col: i32, row: i32) -> Self {
        log::info!("GridItem::new: 创建格子物品，类型 {:?}，位置 ({}, {})", item_type, col, row);
        Self {
            grid_item_type: item_type,
            grid_item_state: GridItemState::Normal,
            x: 0,
            y: 0,
            col,
            row,
            active: true,
            counter: 0,
            anim_counter: 0,
        }
    }

    pub fn update(&mut self) {
        log::trace!("GridItem::update: 更新格子物品，类型 {:?}，位置 ({}, {})", self.grid_item_type, self.col, self.row);
        if !self.active {
            log::trace!("GridItem::update: 格子物品未激活，跳过更新");
            return;
        }
        self.counter += 1;
        self.anim_counter += 1;
    }

    pub fn draw(&self) {
        log::trace!("GridItem::draw: 绘制格子物品，类型 {:?}", self.grid_item_type);
        // TODO: 绘制格子物品
    }
}
