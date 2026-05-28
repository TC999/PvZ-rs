// 对应 C++ 中的 ReanimationLawn.h / ReanimationLawn.cpp
// 草坪动画扩展

use crate::const_enums::*;
use crate::todlib::reanimator::Reanimation;

pub struct ReanimationLawn {
    pub reanim: Reanimation,
    pub lawn_specific_data: i32,
}

impl ReanimationLawn {
    pub fn new(reanim_type: ReanimationType) -> Self {
        Self {
            reanim: Reanimation::new(reanim_type),
            lawn_specific_data: 0,
        }
    }

    pub fn update(&mut self) {
        self.reanim.update();
    }

    pub fn draw(&self) {
        self.reanim.draw();
    }
}
