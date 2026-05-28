// 对应 C++ 中的 Coin.h / Coin.cpp

use crate::const_enums::*;

pub struct Coin {
    pub coin_type: CoinType,
    pub coin_motion: CoinMotion,
    pub x: i32,
    pub y: i32,
    pub value: i32,
    pub active: bool,
    pub collected: bool,
    pub life: i32,
    pub vel_x: f32,
    pub vel_y: f32,
}

impl Coin {
    pub fn new(coin_type: CoinType, x: i32, y: i32) -> Self {
        log::info!("Coin::new: 创建金币，类型 {:?}，位置 ({}, {})", coin_type, x, y);
        Self {
            coin_type,
            coin_motion: CoinMotion::Coin,
            x,
            y,
            value: 0,
            active: true,
            collected: false,
            life: 300,
            vel_x: 0.0,
            vel_y: 0.0,
        }
    }

    pub fn update(&mut self) {
        log::trace!("Coin::update: 更新金币，类型 {:?}，位置 ({}, {})", self.coin_type, self.x, self.y);
        if !self.active {
            log::trace!("Coin::update: 金币未激活，跳过更新");
            return;
        }
        self.x = (self.x as f32 + self.vel_x) as i32;
        self.y = (self.y as f32 + self.vel_y) as i32;
        self.life -= 1;
        if self.life <= 0 {
            log::info!("Coin::update: 金币生命结束，设置为已收集");
            self.collected = true;
        }
    }
}
