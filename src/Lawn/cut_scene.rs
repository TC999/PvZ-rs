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
        log::info!("CutScene::new: 创建过场动画，类型 {}", cutscene_type);
        Self {
            cutscene_type,
            active: false,
            time: 0,
            duration: 300,
            state: 0,
        }
    }

    pub fn start(&mut self) {
        log::info!("CutScene::start: 启动过场动画，类型 {}", self.cutscene_type);
        self.active = true;
        self.time = 0;
    }

    pub fn update(&mut self) {
        log::trace!("CutScene::update: 更新过场动画，类型 {}，时间 {}/{}", self.cutscene_type, self.time, self.duration);
        if !self.active {
            log::trace!("CutScene::update: 过场动画未激活，跳过更新");
            return;
        }
        self.time += 1;
        if self.time >= self.duration {
            log::info!("CutScene::update: 过场动画完成");
            self.active = false;
        }
    }
}
