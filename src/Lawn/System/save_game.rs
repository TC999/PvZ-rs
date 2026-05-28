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
        log::info!("SaveGame::new: 创建存档系统，路径 {}", save_path);
        Self {
            save_path: save_path.to_string(),
        }
    }

    pub fn save_player_info(&self, player: &PlayerInfo) -> bool {
        log::info!("SaveGame::save_player_info: 保存玩家信息，名称 {}", player.name);
        let path = format!("{}/{}.dat", self.save_path, player.name);
        // TODO: 实际序列化保存
        let content = format!(
            "name={}\ncoins={}\ncurrent_level={}\nadventure_completed={}\n",
            player.name, player.coins, player.current_level, player.adventure_completed
        );
        let result = fs::write(&path, content).is_ok();
        if result {
            log::info!("SaveGame::save_player_info: 玩家信息保存成功，路径 {}", path);
        } else {
            log::error!("SaveGame::save_player_info: 玩家信息保存失败，路径 {}", path);
        }
        result
    }

    pub fn load_player_info(&self, name: &str) -> Option<PlayerInfo> {
        log::info!("SaveGame::load_player_info: 加载玩家信息，名称 {}", name);
        let path = format!("{}/{}.dat", self.save_path, name);
        let _content = fs::read_to_string(&path).ok()?;
        // TODO: 实际反序列化
        log::info!("SaveGame::load_player_info: 玩家信息加载成功，名称 {}", name);
        Some(PlayerInfo::new(name))
    }

    pub fn save_screenshot(&self, _data: &[u8], name: &str) -> bool {
        log::info!("SaveGame::save_screenshot: 保存截图，名称 {}", name);
        // TODO: 保存关卡截图
        true
    }
}
