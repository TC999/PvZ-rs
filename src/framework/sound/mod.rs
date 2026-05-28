// Sound - 音频系统模块
// 对应 C++ 中的 src/SexyAppFramework/sound/
//
// C++ 依赖: SDL2_mixer, SDL2_mixer_ext
// Rust 映射: sdl2 crate (mixer feature)
// ASSUMPTION: C++ 使用 SDL2_mixer_ext 扩展版，Rust 使用标准 sdl2::mixer

pub mod sound_manager;
pub mod music_interface;
