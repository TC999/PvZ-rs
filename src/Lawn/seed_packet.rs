// 对应 C++ 中的 SeedPacket.h / SeedPacket.cpp

use crate::const_enums::*;

pub struct SeedPacket {
    pub seed_type: SeedType,
    pub x: i32,
    pub y: i32,
    pub cooldown: i32,
    pub max_cooldown: i32,
    pub refresh_countdown: i32,
    pub chosen_seed_state: ChosenSeedState,
    pub active: bool,
    pub selected: bool,
    pub slot_machine_count: i32,
}

impl SeedPacket {
    pub fn new(seed_type: SeedType) -> Self {
        log::info!("SeedPacket::new: 创建种子包，类型 {:?}", seed_type);
        Self {
            seed_type,
            x: 0,
            y: 0,
            cooldown: 0,
            max_cooldown: 100,
            refresh_countdown: 0,
            chosen_seed_state: ChosenSeedState::InChooser,
            active: true,
            selected: false,
            slot_machine_count: 0,
        }
    }

    pub fn update(&mut self) {
        log::trace!("SeedPacket::update: 更新种子包，类型 {:?}，冷却 {}", self.seed_type, self.cooldown);
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }
        if self.refresh_countdown > 0 {
            self.refresh_countdown -= 1;
        }
    }
}
