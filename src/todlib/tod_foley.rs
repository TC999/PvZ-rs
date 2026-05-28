// 对应 C++ 中的 TodFoley.h / TodFoley.cpp
// 音效系统

pub struct FoleyInstance {
    pub sound_id: i32,
    pub volume: f32,
    pub pan: f32,
    pub pitch: f32,
    pub looping: bool,
    pub playing: bool,
}

impl FoleyInstance {
    pub fn new(sound_id: i32) -> Self {
        log::debug!("FoleyInstance::new: 创建音效实例，ID {}", sound_id);
        Self {
            sound_id,
            volume: 1.0,
            pan: 0.0,
            pitch: 1.0,
            looping: false,
            playing: false,
        }
    }

    pub fn play(&mut self) {
        log::info!("FoleyInstance::play: 播放音效，ID {}", self.sound_id);
        self.playing = true;
    }

    pub fn stop(&mut self) {
        log::info!("FoleyInstance::stop: 停止音效，ID {}", self.sound_id);
        self.playing = false;
    }
}

// 对应 C++ 中的 TodFoley
pub struct TodFoley {
    pub instances: Vec<FoleyInstance>,
}

impl TodFoley {
    pub fn new() -> Self {
        log::debug!("TodFoley::new: 创建音效系统");
        Self {
            instances: Vec::new(),
        }
    }

    pub fn play(&mut self, sound_id: i32) -> usize {
        log::info!("TodFoley::play: 播放音效，ID {}，当前实例数 {}", sound_id, self.instances.len());
        let instance = FoleyInstance::new(sound_id);
        self.instances.push(instance);
        self.instances.len() - 1
    }

    pub fn stop_all(&mut self) {
        log::info!("TodFoley::stop_all: 停止所有音效，实例数 {}", self.instances.len());
        for inst in &mut self.instances {
            inst.stop();
        }
    }

    pub fn update(&mut self) {
        log::trace!("TodFoley::update: 更新音效系统，实例数 {}", self.instances.len());
        // TODO: 更新音效状态
    }
}
