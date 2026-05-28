// 对应 C++ 中的 MusicInterface.h / SDLMusicInterface.h / SDLMusicInterface.cpp
// 音乐播放接口
//
// C++ 依赖: SDL2_mixer (Mix_Music)
// Rust 映射: sdl2::mixer::Music

/// MusicInterface - 音乐接口基类 (对应 C++ Sexy::MusicInterface)
pub struct MusicInterface {
    pub global_volume: i32,
}

impl MusicInterface {
    pub fn new() -> Self {
        Self { global_volume: 100 }
    }

    pub fn set_volume(&mut self, volume: f64) {
        self.global_volume = (volume * 100.0).clamp(0.0, 100.0) as i32;
    }

    pub fn get_volume(&self) -> i32 { self.global_volume }
}

/// SDLMusicInfo - 音乐轨道信息 (对应 C++ Sexy::SDLMusicInfo)
pub struct MusicInfo {
    pub volume: f64,
    pub volume_add: f64,
    pub volume_cap: f64,
    pub stop_on_fade: bool,
}

impl MusicInfo {
    pub fn new() -> Self {
        Self { volume: 1.0, volume_add: 0.0, volume_cap: 1.0, stop_on_fade: false }
    }
}
