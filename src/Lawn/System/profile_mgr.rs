// 对应 C++ 中的 ProfileMgr.h / ProfileMgr.cpp
// 用户档案管理器

use super::player_info::PlayerInfo;
use std::collections::HashMap;

pub struct ProfileMgr {
    pub profiles: HashMap<String, PlayerInfo>,
    pub current_profile: String,
    pub profile_path: String,
}

impl ProfileMgr {
    pub fn new(profile_path: &str) -> Self {
        Self {
            profiles: HashMap::new(),
            current_profile: String::new(),
            profile_path: profile_path.to_string(),
        }
    }

    pub fn create_profile(&mut self, name: &str) -> &PlayerInfo {
        let profile = PlayerInfo::new(name);
        self.profiles.insert(name.to_string(), profile);
        self.current_profile = name.to_string();
        self.profiles.get(name).unwrap()
    }

    pub fn get_current_profile(&self) -> Option<&PlayerInfo> {
        self.profiles.get(&self.current_profile)
    }

    pub fn get_current_profile_mut(&mut self) -> Option<&mut PlayerInfo> {
        self.profiles.get_mut(&self.current_profile)
    }

    pub fn delete_profile(&mut self, name: &str) -> bool {
        self.profiles.remove(name).is_some()
    }

    pub fn switch_profile(&mut self, name: &str) -> bool {
        if self.profiles.contains_key(name) {
            self.current_profile = name.to_string();
            true
        } else {
            false
        }
    }

    pub fn save(&self) {
        // TODO: 实现档案保存
    }

    pub fn load(&mut self) {
        // TODO: 实现档案加载
    }
}
