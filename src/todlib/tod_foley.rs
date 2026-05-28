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
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }
}

// 对应 C++ 中的 TodFoley
pub struct TodFoley {
    pub instances: Vec<FoleyInstance>,
}

impl TodFoley {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
        }
    }

    pub fn play(&mut self, sound_id: i32) -> usize {
        let instance = FoleyInstance::new(sound_id);
        self.instances.push(instance);
        self.instances.len() - 1
    }

    pub fn stop_all(&mut self) {
        for inst in &mut self.instances {
            inst.stop();
        }
    }

    pub fn update(&mut self) {
        // TODO: 更新音效状态
    }
}
