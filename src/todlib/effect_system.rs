// 对应 C++ 中的 EffectSystem.h / EffectSystem.cpp
// 特效系统

use crate::const_enums::*;
use crate::todlib::attachment::Attachment;
use crate::todlib::reanimator::Reanimation;
use crate::todlib::tod_particle::TodParticleSystem;
use crate::todlib::trail::Trail;

// 对应 C++ 中的 EffectSystem
pub struct EffectSystem {
    pub particle_systems: Vec<TodParticleSystem>,
    pub reanimations: Vec<Reanimation>,
    pub trails: Vec<Trail>,
    pub attachments: Vec<Attachment>,
}

impl EffectSystem {
    pub fn new() -> Self {
        log::debug!("EffectSystem::new: 创建特效系统");
        Self {
            particle_systems: Vec::new(),
            reanimations: Vec::new(),
            trails: Vec::new(),
            attachments: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        log::trace!("EffectSystem::update: 更新特效系统，粒子系统 {} 个，动画 {} 个，轨迹 {} 个，附件 {} 个",
            self.particle_systems.len(), self.reanimations.len(), self.trails.len(), self.attachments.len());
        // TODO: 更新所有特效
    }

    pub fn draw(&self) {
        log::trace!("EffectSystem::draw: 绘制特效系统");
        // TODO: 绘制所有特效
    }
}

impl Default for EffectSystem {
    fn default() -> Self {
        Self::new()
    }
}
