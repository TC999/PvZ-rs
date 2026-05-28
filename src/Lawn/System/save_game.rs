// 对应 C++ 中的 SaveGame.h / SaveGame.cpp
// 存档系统

use super::player_info::PlayerInfo;
use std::fs;
use std::path::Path;

pub struct SaveGame {
    save_path: String,
}

impl SaveGame {
    pub fn new(save_path: &str) -> Self {
        Self {
            save_path: save_path.to_string(),
        }
    }

    pub fn save_player_info(&self, player: &PlayerInfo) -> bool {
        let path = format!("{}/{}.dat", self.save_path, player.name);
        // TODO: 实际序列化保存
        let content = format!(
            "name={}\ncoins={}\ncurrent_level={}\nadventure_completed={}\n",
            player.name, player.coins, player.current_level, player.adventure_completed
        );
        fs::write(&path, content).is_ok()
    }

    pub fn load_player_info(&self, name: &str) -> Option<PlayerInfo> {
        let path = format!("{}/{}.dat", self.save_path, name);
        let _content = fs::read_to_string(&path).ok()?;
        // TODO: 实际反序列化
        Some(PlayerInfo::new(name))
    }

    pub fn save_screenshot(&self, _data: &[u8], _name: &str) -> bool {
        // TODO: 保存关卡截图
        true
    }
}
