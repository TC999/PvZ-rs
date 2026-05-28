// 对应 C++ 中的 PoolEffect.h / PoolEffect.cpp
// 泳池水波效果

pub struct PoolEffect {
    pub active: bool,
    pub wave_offset: f32,
    pub wave_speed: f32,
    pub amplitude: f32,
    pub frequency: f32,
}

impl PoolEffect {
    pub fn new() -> Self {
        log::debug!("PoolEffect::new: 创建泳池水波效果");
        Self {
            active: false,
            wave_offset: 0.0,
            wave_speed: 0.05,
            amplitude: 3.0,
            frequency: 0.02,
        }
    }

    pub fn update(&mut self) {
        log::trace!("PoolEffect::update: 更新水波效果，active={}", self.active);
        if !self.active {
            log::trace!("PoolEffect::update: 水波效果未激活，跳过更新");
            return;
        }
        self.wave_offset += self.wave_speed;
    }

    pub fn get_wave_y(&self, x: f32) -> f32 {
        let result = (x * self.frequency + self.wave_offset).sin() * self.amplitude;
        log::trace!("PoolEffect::get_wave_y: x={}, 结果 {}", x, result);
        result
    }
}
