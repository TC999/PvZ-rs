// 对应 C++ 中的 CutScene.h / CutScene.cpp

use crate::const_enums::*;

pub struct CutScene {
    pub cutscene_type: i32,
    pub active: bool,
    pub time: i32,
    pub duration: i32,
    pub state: i32,
}

impl CutScene {
    pub fn new(cutscene_type: i32) -> Self {
        Self {
            cutscene_type,
            active: false,
            time: 0,
            duration: 300,
            state: 0,
        }
    }

    pub fn start(&mut self) {
        self.active = true;
        self.time = 0;
    }

    pub fn update(&mut self) {
        if !self.active { return; }
        self.time += 1;
        if self.time >= self.duration {
            self.active = false;
        }
    }
}
