// 对应 C++ 中的 SoundManager.h / SDLSoundManager.h / SDLSoundManager.cpp
// 音效管理器
//
// C++ 依赖: SDL2_mixer (Mix_Chunk, Mix_PlayChannel 等)
// Rust 映射: sdl2::mixer
// ASSUMPTION: C++ 的 SDL_mixer_ext 功能在 Rust 中使用标准 sdl2::mixer

use std::collections::HashMap;

const MAX_SOURCE_SOUNDS: usize = 4096;
const MAX_CHANNELS: usize = 32;

/// SoundManager - 音效管理器基类 (对应 C++ Sexy::SoundManager)
pub struct SoundManager {
    pub initialized: bool,
    pub master_volume: f64,
}

impl SoundManager {
    pub fn new() -> Self {
        Self { initialized: false, master_volume: 1.0 }
    }

    pub fn is_initialized(&self) -> bool { self.initialized }

    pub fn set_volume(&mut self, volume: f64) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_master_volume(&self) -> f64 { self.master_volume }

    pub fn set_master_volume(&mut self, volume: f64) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }
}

/// SDLSoundInstance - 音效播放实例 (对应 C++ Sexy::SDLSoundInstance)
pub struct SoundInstance {
    pub sound_id: i32,
    pub channel: i32,
    pub volume: f64,
    pub pan: i32,
    pub released: bool,
}

impl SoundInstance {
    pub fn new() -> Self {
        Self { sound_id: -1, channel: -1, volume: 1.0, pan: 0, released: false }
    }

    pub fn release(&mut self) { self.released = true; }

    pub fn is_released(&self) -> bool { self.released }

    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_pan(&mut self, pan: i32) {
        self.pan = pan.clamp(-100, 100);
    }
}
