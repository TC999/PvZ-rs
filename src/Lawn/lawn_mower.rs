// 对应 C++ 中的 LawnMower.h / LawnMower.cpp

use crate::const_enums::*;

pub struct LawnMower {
    pub lawn_mower_type: LawnMowerType,
    pub state: LawnMowerState,
    pub x: i32,
    pub y: i32,
    pub row: i32,
    pub active: bool,
    pub visible: bool,
    pub anim_counter: i32,
    pub speed: f32,
    pub height: MowerHeight,
}

impl LawnMower {
    pub fn new(mower_type: LawnMowerType, row: i32) -> Self {
        log::info!("LawnMower::new: 创建割草机，类型 {:?}，行 {}", mower_type, row);
        Self {
            lawn_mower_type: mower_type,
            state: LawnMowerState::Ready,
            x: 0,
            y: 0,
            row,
            active: true,
            visible: true,
            anim_counter: 0,
            speed: 3.0,
            height: MowerHeight::Land,
        }
    }

    pub fn update(&mut self) {
        log::trace!("LawnMower::update: 更新割草机，类型 {:?}，行 {}，状态 {:?}", self.lawn_mower_type, self.row, self.state);
        if !self.active {
            log::trace!("LawnMower::update: 割草机未激活，跳过更新");
            return;
        }
        match self.state {
            LawnMowerState::Triggered => {
                log::trace!("LawnMower::update: 割草机已触发，移动中，位置 x={}", self.x);
                self.x = (self.x as f32 + self.speed) as i32;
            }
            _ => {}
        }
        self.anim_counter += 1;
    }

    pub fn trigger(&mut self) {
        log::info!("LawnMower::trigger: 触发割草机，行 {}", self.row);
        self.state = LawnMowerState::Triggered;
    }
}
