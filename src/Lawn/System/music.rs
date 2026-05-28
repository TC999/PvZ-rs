// 对应 C++ 中的 Music.h / Music.cpp
// 音乐系统

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicFile {
    None = -1,
    MainMenu = 0,
    ChooseYourSeeds = 1,
    Grasswalk = 2,
    Moongrains = 3,
    WateryGraves = 4,
    RigorMormist = 5,
    Cerebrawl = 6,
    GrazeTheRoof = 7,
    BrainiacManiac = 8,
    Loonboon = 9,
    UltimateBattle = 10,
    ZenGarden = 11,
    WateryGravesFast = 12,
    RigorMormistFast = 13,
    CerebrawlFast = 14,
    GrazeTheRoofFast = 15,
    LoonboonFast = 16,
    UltimateBattleFast = 17,
    CreditsZombies = 18,
    CrazyDave = 19,
    CrazyDaveShort = 20,
    CrazyDaveLong = 21,
    Fog = 22,
    Pool = 23,
    Roof = 24,
    MiniGame = 25,
}

pub struct Music {
    pub current_music: MusicFile,
    pub volume: f32,
    pub enabled: bool,
    pub fading_out: bool,
    pub fade_alpha: f32,
}

impl Music {
    pub fn new() -> Self {
        log::debug!("Music::new: 创建音乐系统");
        Self {
            current_music: MusicFile::None,
            volume: 1.0,
            enabled: true,
            fading_out: false,
            fade_alpha: 1.0,
        }
    }

    pub fn play(&mut self, music: MusicFile) {
        log::info!("Music::play: 播放音乐 {:?}", music);
        self.current_music = music;
    }

    pub fn stop(&mut self) {
        log::info!("Music::stop: 停止音乐");
        self.current_music = MusicFile::None;
    }

    pub fn update(&mut self) {
        log::trace!("Music::update: 更新音乐系统，当前音乐 {:?}，淡出状态 {}", self.current_music, self.fading_out);
        if self.fading_out {
            self.fade_alpha -= 0.01;
            if self.fade_alpha <= 0.0 {
                log::info!("Music::update: 淡出完成");
                self.fading_out = false;
                self.fade_alpha = 0.0;
            }
        }
    }
}
