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
        Self {
            current_music: MusicFile::None,
            volume: 1.0,
            enabled: true,
            fading_out: false,
            fade_alpha: 1.0,
        }
    }

    pub fn play(&mut self, music: MusicFile) {
        self.current_music = music;
    }

    pub fn stop(&mut self) {
        self.current_music = MusicFile::None;
    }

    pub fn update(&mut self) {
        if self.fading_out {
            self.fade_alpha -= 0.01;
            if self.fade_alpha <= 0.0 {
                self.fading_out = false;
                self.fade_alpha = 0.0;
            }
        }
    }
}
