// 对应 C++ 中的 ModVal.h / ModVal.cpp
// 值修饰器 - 用于平滑过渡和动画值计算
use crate::const_enums::TodCurves;

pub struct ModVal {
    pub value: f32,
    pub target: f32,
    pub start_value: f32,
    pub duration: f32,
    pub elapsed: f32,
    pub curve: TodCurves,
    pub active: bool,
}

impl ModVal {
    pub fn new(curve: TodCurves) -> Self {
        Self {
            value: 0.0,
            target: 0.0,
            start_value: 0.0,
            duration: 0.0,
            elapsed: 0.0,
            curve,
            active: false,
        }
    }

    pub fn set_target(&mut self, target: f32, duration: f32) {
        self.start_value = self.value;
        self.target = target;
        self.duration = duration;
        self.elapsed = 0.0;
        self.active = true;
    }

    pub fn update(&mut self, dt: f32) {
        if !self.active { return; }
        self.elapsed += dt;
        let t = if self.duration > 0.0 {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        } else {
            1.0
        };

        self.value = self.start_value + (self.target - self.start_value) * self.apply_curve(t);

        if t >= 1.0 {
            self.active = false;
            self.value = self.target;
        }
    }

    fn apply_curve(&self, t: f32) -> f32 {
        match self.curve {
            TodCurves::Constant => 0.0,
            TodCurves::Linear => t,
            TodCurves::EaseIn => t * t,
            TodCurves::EaseOut => t * (2.0 - t),
            TodCurves::EaseInOut => if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t },
            _ => t, // 其他曲线暂时简化
        }
    }
}
