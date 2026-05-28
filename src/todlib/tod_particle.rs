// 对应 C++ 中的 TodParticle.h / TodParticle.cpp
// 粒子系统

use crate::const_enums::*;

pub struct TodParticle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub max_life: f32,
    pub size: f32,
    pub start_size: f32,
    pub end_size: f32,
    pub color_start: (u8, u8, u8, u8),
    pub color_end: (u8, u8, u8, u8),
    pub particle_type: ParticleEffect,
    pub dead: bool,
}

impl TodParticle {
    pub fn new(particle_type: ParticleEffect, x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            life: 1.0,
            max_life: 1.0,
            size: 10.0,
            start_size: 10.0,
            end_size: 0.0,
            color_start: (255, 255, 255, 255),
            color_end: (255, 255, 255, 0),
            particle_type,
            dead: false,
        }
    }

    pub fn update(&mut self) {
        if self.dead { return; }
        self.life -= 0.01;
        self.x += self.vx;
        self.y += self.vy;
        if self.life <= 0.0 {
            self.dead = true;
        }
    }
}

pub struct ParticleEmitter {
    pub emitter_type: EmitterType,
    pub particle_type: ParticleEffect,
    pub x: f32,
    pub y: f32,
    pub rate: f32,
    pub countdown: f32,
    pub active: bool,
}

impl ParticleEmitter {
    pub fn new(emitter_type: EmitterType, particle_type: ParticleEffect) -> Self {
        Self {
            emitter_type,
            particle_type,
            x: 0.0,
            y: 0.0,
            rate: 0.1,
            countdown: 0.0,
            active: true,
        }
    }

    pub fn update(&mut self) -> Vec<TodParticle> {
        let mut new_particles = Vec::new();
        if !self.active { return new_particles; }
        self.countdown -= 1.0;
        if self.countdown <= 0.0 {
            self.countdown = self.rate;
            new_particles.push(TodParticle::new(self.particle_type, self.x, self.y));
        }
        new_particles
    }
}

pub struct TodParticleSystem {
    pub particles: Vec<TodParticle>,
    pub emitters: Vec<ParticleEmitter>,
    pub dead: bool,
    pub particle_type: ParticleEffect,
}

impl TodParticleSystem {
    pub fn new(particle_type: ParticleEffect) -> Self {
        Self {
            particles: Vec::new(),
            emitters: Vec::new(),
            dead: false,
            particle_type,
        }
    }

    pub fn update(&mut self) {
        // 更新现有粒子
        for p in &mut self.particles {
            p.update();
        }
        self.particles.retain(|p| !p.dead);

        // 更新发射器
        for emitter in &mut self.emitters {
            let new_particles = emitter.update();
            self.particles.extend(new_particles);
        }
    }

    pub fn draw(&self) {
        // TODO: 绘制粒子
    }
}
