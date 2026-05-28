// 对应 C++ 中的 Plant.h / Plant.cpp
// 植物类

use crate::const_enums::*;

pub struct Plant {
    pub plant_type: SeedType,
    pub x: i32,
    pub y: i32,
    pub row: i32,
    pub col: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub state: PlantPriority,
    pub active: bool,
    pub shoot_countdown: i32,
    pub anim_countdown: i32,
}

impl Plant {
    pub fn new(plant_type: SeedType, col: i32, row: i32) -> Self {
        log::info!("Plant::new: 创建植物，类型 {:?}，位置 ({}, {})", plant_type, col, row);
        Self {
            plant_type,
            x: 0,
            y: 0,
            row,
            col,
            hp: 300,
            max_hp: 300,
            state: PlantPriority::Any,
            active: true,
            shoot_countdown: 0,
            anim_countdown: 0,
        }
    }

    pub fn update(&mut self) {
        log::trace!("Plant::update: 更新植物，类型 {:?}，位置 ({}, {})", self.plant_type, self.col, self.row);
        // TODO: 实现植物更新逻辑
        if self.shoot_countdown > 0 {
            self.shoot_countdown -= 1;
            log::trace!("Plant::update: 射击倒计时减少到 {}", self.shoot_countdown);
        }
    }

    pub fn damage(&mut self, amount: i32) {
        log::debug!("Plant::damage: 植物受到 {} 点伤害，当前生命值 {}", amount, self.hp);
        // TODO: 实现伤害计算
    }

    pub fn is_dead(&self) -> bool {
        let result = !self.active || self.hp <= 0;
        log::trace!("Plant::is_dead: 植物状态 active={}, hp={}，结果 {}", self.active, self.hp, result);
        result
    }
}
