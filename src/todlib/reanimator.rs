// 对应 C++ 中的 Reanimator.h / Reanimator.cpp
// 动画系统 - 基于时间轴的精灵动画

use crate::const_enums::*;
use crate::todlib::reanim_atlas::ReanimAtlas;

#[derive(Debug, Clone)]
pub struct ReanimatorTrack {
    pub track_name: String,
    pub transform: (f32, f32, f32, f32, f32, f32), // x, y, scale_x, scale_y, rotation, alpha
    pub visible: bool,
}

#[derive(Debug, Clone)]
pub struct ReanimatorTransform {
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub alpha: f32,
}

impl Default for ReanimatorTransform {
    fn default() -> Self {
        Self {
            x: 0.0, y: 0.0,
            scale_x: 1.0, scale_y: 1.0,
            rotation: 0.0, alpha: 1.0,
        }
    }
}

// 对应 C++ 中的 Reanimation 类
pub struct Reanimation {
    pub reanim_type: ReanimationType,
    pub x: f32,
    pub y: f32,
    pub loop_type: ReanimLoopType,
    pub loop_count: i32,
    pub frame_count: f32,
    pub frame_start: f32,
    pub anim_rate: f32,
    pub dead: bool,
    pub atlas: Option<ReanimAtlas>,
    pub tracks: Vec<ReanimatorTrack>,
    pub current_frame: i32,
    pub elapsed_time: f32,
    pub transform: ReanimatorTransform,
    pub draw_group: i32,
}

impl Reanimation {
    pub fn new(reanim_type: ReanimationType) -> Self {
        log::info!("Reanimation::new: 创建动画，类型 {:?}", reanim_type);
        Self {
            reanim_type,
            x: 0.0,
            y: 0.0,
            loop_type: ReanimLoopType::Loop,
            loop_count: 0,
            frame_count: 0.0,
            frame_start: 0.0,
            anim_rate: 1.0,
            dead: false,
            atlas: None,
            tracks: Vec::new(),
            current_frame: 0,
            elapsed_time: 0.0,
            transform: ReanimatorTransform::default(),
            draw_group: 0,
        }
    }

    pub fn update(&mut self) {
        log::trace!("Reanimation::update: 更新动画，类型 {:?}，帧 {}", self.reanim_type, self.current_frame);
        // TODO: 实现动画帧更新
        if self.dead {
            log::trace!("Reanimation::update: 动画已死亡，跳过更新");
            return;
        }
        self.elapsed_time += self.anim_rate;
    }

    pub fn draw(&self) {
        log::trace!("Reanimation::draw: 绘制动画，类型 {:?}，帧 {}", self.reanim_type, self.current_frame);
        // TODO: 实现动画绘制
    }

    pub fn is_dead(&self) -> bool {
        log::trace!("Reanimation::is_dead: 动画状态 dead={}", self.dead);
        self.dead
    }
}

// 对应 C++ 中的 ReanimatorCache - 预加载和管理动画资源
pub struct ReanimatorCache {
    pub animations: Vec<Reanimation>,
}

impl ReanimatorCache {
    pub fn new() -> Self {
        log::debug!("ReanimatorCache::new: 创建动画缓存");
        Self {
            animations: Vec::new(),
        }
    }

    pub fn load_animation(&mut self, _filename: &str) -> Option<usize> {
        // TODO: 加载动画文件
        None
    }

    pub fn get_animation(&self, index: usize) -> Option<&Reanimation> {
        self.animations.get(index)
    }
}
