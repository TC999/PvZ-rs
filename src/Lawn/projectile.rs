// 对应 C++ 中的 Projectile.h / Projectile.cpp

use crate::const_enums::*;

pub struct Projectile {
    pub projectile_type: ProjectileType,
    pub motion: ProjectileMotion,
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub row: i32,
    pub damage: i32,
    pub active: bool,
    pub dead: bool,
    pub anim_counter: i32,
    pub shadow: bool,
    pub target_x: f32,
    pub target_y: f32,
}

impl Projectile {
    pub fn new(projectile_type: ProjectileType, row: i32) -> Self {
        log::info!("Projectile::new: 创建投射物，类型 {:?}，行 {}", projectile_type, row);
        Self {
            projectile_type,
            motion: ProjectileMotion::Straight,
            x: 0.0,
            y: 0.0,
            vel_x: 0.0,
            vel_y: 0.0,
            row,
            damage: 20,
            active: true,
            dead: false,
            anim_counter: 0,
            shadow: false,
            target_x: 0.0,
            target_y: 0.0,
        }
    }

    pub fn update(&mut self) {
        log::trace!("Projectile::update: 更新投射物，类型 {:?}，位置 ({}, {})", self.projectile_type, self.x, self.y);
        if !self.active {
            log::trace!("Projectile::update: 投射物未激活，跳过更新");
            return;
        }
        self.x += self.vel_x;
        self.y += self.vel_y;
        self.anim_counter += 1;
        // 离开屏幕则标记死亡
        if self.x > 850.0 || self.x < -50.0 || self.y > 650.0 || self.y < -50.0 {
            log::info!("Projectile::update: 投射物离开屏幕，标记为死亡");
            self.dead = true;
        }
    }
}
