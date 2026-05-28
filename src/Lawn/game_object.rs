// 对应 C++ 中的 GameObject.h / GameObject.cpp

use crate::const_enums::*;

pub struct GameObject {
    pub object_type: GameObjectType,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
    pub row: i32,
    pub layer: RenderLayer,
    pub id: usize,
    pub active: bool,
    pub update_order: i32,
}

impl GameObject {
    pub fn new(object_type: GameObjectType) -> Self {
        Self {
            object_type,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            visible: true,
            row: 0,
            layer: RenderLayer::Lawn,
            id: 0,
            active: true,
            update_order: 0,
        }
    }

    pub fn update(&mut self) {
        // 基类更新
    }

    pub fn draw(&self) {
        // 基类绘制
    }
}
