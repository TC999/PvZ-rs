// 对应 C++ 中的 PlayerInfo.h / PlayerInfo.cpp
// 玩家信息

pub struct PlayerInfo {
    pub name: String,
    pub coins: i32,
    pub adventure_completed: bool,
    pub current_level: i32,
    pub level_completed: [bool; 50],
    pub zen_garden_enabled: bool,
    pub mini_games_unlocked: Vec<bool>,
    pub puzzle_modes_unlocked: Vec<bool>,
    pub survival_modes_unlocked: Vec<bool>,
    pub num_lawn_mowers: i32,
    pub purchased_items: Vec<bool>,
    pub challenges_completed: Vec<bool>,
    pub trophies: Vec<bool>,
}

impl PlayerInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            coins: 0,
            adventure_completed: false,
            current_level: 1,
            level_completed: [false; 50],
            zen_garden_enabled: false,
            mini_games_unlocked: vec![false; 20],
            puzzle_modes_unlocked: vec![false; 20],
            survival_modes_unlocked: vec![false; 20],
            num_lawn_mowers: 3,
            purchased_items: vec![false; 32],
            challenges_completed: vec![false; 100],
            trophies: vec![false; 100],
        }
    }

    pub fn add_coins(&mut self, amount: i32) {
        self.coins += amount;
    }

    pub fn complete_level(&mut self, level: i32) {
        if level >= 1 && level <= 50 {
            self.level_completed[(level - 1) as usize] = true;
            if level > self.current_level {
                self.current_level = level;
            }
        }
    }

    pub fn is_level_completed(&self, level: i32) -> bool {
        if level >= 1 && level <= 50 {
            self.level_completed[(level - 1) as usize]
        } else {
            false
        }
    }
}
