// 对应 C++ 中的 Attachment.h / Attachment.cpp
// 附件 - 用于物体附加特效（如装饰等）

use crate::const_enums::EffectType;

pub struct Attachment {
    pub effect_type: EffectType,
    pub dead: bool,
}

impl Attachment {
    pub fn new() -> Self {
        Self {
            effect_type: EffectType::Attachment,
            dead: false,
        }
    }

    pub fn update(&mut self) {
        // TODO: 实现附件更新逻辑
    }

    pub fn draw(&self) {
        // TODO: 实现附件绘制逻辑
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }
}
