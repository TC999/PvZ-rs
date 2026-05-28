// 对应 C++ 中的 Trail.h / Trail.cpp
// 拖尾效果

use crate::const_enums::*;

pub struct TrailPoint {
    pub x: f32,
    pub y: f32,
    pub alpha: f32,
}

pub struct Trail {
    pub points: Vec<TrailPoint>,
    pub max_points: usize,
    pub trail_type: ParticleEffect,
    pub width: f32,
    pub color: (u8, u8, u8, u8),
    pub dead: bool,
    pub duration: f32,
    pub age: f32,
}

impl Trail {
    pub fn new(trail_type: ParticleEffect, max_points: usize) -> Self {
        log::debug!("Trail::new: 创建拖尾效果，类型 {:?}，最大点数 {}", trail_type, max_points);
        Self {
            points: Vec::new(),
            max_points,
            trail_type,
            width: 3.0,
            color: (255, 255, 255, 255),
            dead: false,
            duration: 1.0,
            age: 0.0,
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        log::trace!("Trail::add_point: 添加点 ({}, {})", x, y);
        self.points.push(TrailPoint { x, y, alpha: 1.0 });
        if self.points.len() > self.max_points {
            log::trace!("Trail::add_point: 移除最旧的点");
            self.points.remove(0);
        }
    }

    pub fn update(&mut self) {
        log::trace!("Trail::update: 更新拖尾效果，类型 {:?}，点数 {}，年龄 {}", self.trail_type, self.points.len(), self.age);
        if self.dead {
            log::trace!("Trail::update: 拖尾效果已死亡，跳过更新");
            return;
        }
        self.age += 0.016; // 约 60fps
        for point in &mut self.points {
            point.alpha -= 0.05;
        }
        self.points.retain(|p| p.alpha > 0.0);
        if self.age > self.duration {
            log::info!("Trail::update: 拖尾效果持续时间结束");
            self.dead = true;
        }
    }

    pub fn draw(&self) {
        log::trace!("Trail::draw: 绘制拖尾效果，类型 {:?}", self.trail_type);
        // TODO: 实现拖尾绘制
    }
}
