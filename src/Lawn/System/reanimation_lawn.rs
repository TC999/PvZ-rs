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
        log::info!("ReanimationLawn::new: 创建草坪动画，类型 {:?}", reanim_type);
        Self {
            reanim: Reanimation::new(reanim_type),
            lawn_specific_data: 0,
        }
    }

    pub fn update(&mut self) {
        log::trace!("ReanimationLawn::update: 更新草坪动画");
        self.reanim.update();
    }

    pub fn draw(&self) {
        log::trace!("ReanimationLawn::draw: 绘制草坪动画");
        self.reanim.draw();
    }
}
