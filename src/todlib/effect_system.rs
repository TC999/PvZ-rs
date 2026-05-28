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
        Self {
            particle_systems: Vec::new(),
            reanimations: Vec::new(),
            trails: Vec::new(),
            attachments: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        // TODO: 更新所有特效
    }

    pub fn draw(&self) {
        // TODO: 绘制所有特效
    }
}

impl Default for EffectSystem {
    fn default() -> Self {
        Self::new()
    }
}
