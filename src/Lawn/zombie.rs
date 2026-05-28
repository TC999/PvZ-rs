// 对应 C++ 中的 Zombie.h / Zombie.cpp
// 僵尸类

use crate::const_enums::*;

pub struct Zombie {
    pub zombie_type: ZombieType,
    pub x: i32,
    pub y: i32,
    pub row: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub speed: f64,
    pub phase: ZombiePhase,
    pub height: ZombieHeight,
    pub helm_type: HelmType,
    pub shield_type: ShieldType,
    pub shield_hp: i32,
    pub active: bool,
    pub eating: bool,
    pub frozen: bool,
    pub frozen_countdown: i32,
    pub eat_countdown: i32,
}

impl Zombie {
    pub fn new(zombie_type: ZombieType, row: i32) -> Self {
        log::info!("Zombie::new: 创建僵尸，类型 {:?}，行 {}", zombie_type, row);
        Self {
            zombie_type,
            x: 800, // 从屏幕右侧进入
            y: 0,
            row,
            hp: 200,
            max_hp: 200,
            speed: 0.5,
            phase: ZombiePhase::Normal,
            height: ZombieHeight::Normal,
            helm_type: HelmType::None,
            shield_type: ShieldType::None,
            shield_hp: 0,
            active: true,
            eating: false,
            frozen: false,
            frozen_countdown: 0,
            eat_countdown: 0,
        }
    }

    pub fn update(&mut self) {
        log::trace!("Zombie::update: 更新僵尸，类型 {:?}，行 {}，位置 x={}", self.zombie_type, self.row, self.x);
        if !self.active {
            log::trace!("Zombie::update: 僵尸未激活，跳过更新");
            return;
        }
        // TODO: 实现僵尸移动和更新逻辑
    }

    pub fn damage(&mut self, amount: i32, _flags: DamageFlags) {
        log::debug!("Zombie::damage: 僵尸受到 {} 点伤害，当前生命值 {}", amount, self.hp);
        // TODO: 实现伤害计算
    }

    pub fn is_dead(&self) -> bool {
        let result = !self.active || self.hp <= 0;
        log::trace!("Zombie::is_dead: 僵尸状态 active={}, hp={}，结果 {}", self.active, self.hp, result);
        result
    }
}
