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
        if !self.active { return; }
        match self.state {
            LawnMowerState::Triggered => {
                self.x = (self.x as f32 + self.speed) as i32;
            }
            _ => {}
        }
        self.anim_counter += 1;
    }

    pub fn trigger(&mut self) {
        self.state = LawnMowerState::Triggered;
    }
}
