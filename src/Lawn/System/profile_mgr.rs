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
        log::info!("ProfileMgr::new: 创建档案管理器，路径 {}", profile_path);
        Self {
            profiles: HashMap::new(),
            current_profile: String::new(),
            profile_path: profile_path.to_string(),
        }
    }

    pub fn create_profile(&mut self, name: &str) -> &PlayerInfo {
        log::info!("ProfileMgr::create_profile: 创建档案 {}", name);
        let profile = PlayerInfo::new(name);
        self.profiles.insert(name.to_string(), profile);
        self.current_profile = name.to_string();
        self.profiles.get(name).unwrap()
    }

    pub fn get_current_profile(&self) -> Option<&PlayerInfo> {
        log::trace!("ProfileMgr::get_current_profile: 获取当前档案 {}", self.current_profile);
        self.profiles.get(&self.current_profile)
    }

    pub fn get_current_profile_mut(&mut self) -> Option<&mut PlayerInfo> {
        log::trace!("ProfileMgr::get_current_profile_mut: 获取当前档案的可变引用 {}", self.current_profile);
        self.profiles.get_mut(&self.current_profile)
    }

    pub fn delete_profile(&mut self, name: &str) -> bool {
        log::info!("ProfileMgr::delete_profile: 删除档案 {}", name);
        self.profiles.remove(name).is_some()
    }

    pub fn switch_profile(&mut self, name: &str) -> bool {
        log::info!("ProfileMgr::switch_profile: 切换到档案 {}", name);
        if self.profiles.contains_key(name) {
            self.current_profile = name.to_string();
            true
        } else {
            log::warn!("ProfileMgr::switch_profile: 档案 {} 不存在", name);
            false
        }
    }

    pub fn save(&self) {
        log::info!("ProfileMgr::save: 保存档案（待实现）");
        // TODO: 实现档案保存
    }

    pub fn load(&mut self) {
        log::info!("ProfileMgr::load: 加载档案（待实现）");
        // TODO: 实现档案加载
    }
}
