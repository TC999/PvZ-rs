// 对应 C++ 中的 NewOptionsDialog.h / NewOptionsDialog.cpp
pub struct NewOptionsDialog {
    pub active: bool,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub fullscreen: bool,
    pub hardware_acceleration: bool,
}
impl NewOptionsDialog {
    pub fn new() -> Self { Self { active:false, music_volume:1.0, sfx_volume:1.0, fullscreen:true, hardware_acceleration:true } }
    pub fn update(&mut self) {}
}
