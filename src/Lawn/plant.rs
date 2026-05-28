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
        // TODO: 实现植物更新逻辑
        if self.shoot_countdown > 0 {
            self.shoot_countdown -= 1;
        }
    }

    pub fn damage(&mut self, _amount: i32) {
        // TODO: 实现伤害计算
    }

    pub fn is_dead(&self) -> bool {
        !self.active || self.hp <= 0
    }
}
