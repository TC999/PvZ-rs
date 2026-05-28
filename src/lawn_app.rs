// 对应 C++ 中的 LawnApp.h / LawnApp.cpp
// 应用程序主类

use crate::const_enums::*;
use crate::todlib::reanimator::{Reanimation, ReanimatorCache};
use crate::todlib::effect_system::EffectSystem;
use crate::lawn::board::Board;
use crate::lawn::widget::title_screen::TitleScreen;
use crate::lawn::widget::game_selector::GameSelector;
use crate::lawn::widget::seed_chooser_screen::SeedChooserScreen;
use crate::lawn::widget::award_screen::AwardScreen;
use crate::lawn::widget::credit_screen::CreditScreen;
use crate::lawn::widget::challenge_screen::ChallengeScreen;
use crate::lawn::widget::almanac_dialog::AlmanacDialog;
use crate::lawn::system::music::Music;
use crate::lawn::system::profile_mgr::ProfileMgr;
use crate::lawn::system::player_info::PlayerInfo;
use crate::lawn::system::pool_effect::PoolEffect;
use crate::lawn::system::typing_check::TypingCheck;
use crate::lawn::zen_garden::ZenGarden;
use crate::framework::graphics::graphics::Graphics;

pub struct LevelStats {
    pub unused_lawn_mowers: i32,
}

impl LevelStats {
    pub fn new() -> Self { Self { unused_lawn_mowers: 0 } }
    pub fn reset(&mut self) { self.unused_lawn_mowers = 0; }
}

pub struct LawnApp {
    pub board: Option<Box<Board>>,
    pub title_screen: TitleScreen,
    pub game_selector: GameSelector,
    pub seed_chooser_screen: SeedChooserScreen,
    pub award_screen: AwardScreen,
    pub credit_screen: CreditScreen,
    pub challenge_screen: ChallengeScreen,
    pub effect_system: EffectSystem,
    pub reanimator_cache: ReanimatorCache,
    pub profile_mgr: ProfileMgr,
    pub player_info: Option<PlayerInfo>,
    pub last_level_stats: LevelStats,
    pub close_request: bool,
    pub app_counter: u32,
    pub music: Music,
    pub pool_effect: PoolEffect,
    pub zen_garden: ZenGarden,
    pub game_mode: GameMode,
    pub game_scene: GameScenes,
    pub loading_completed: bool,
    pub first_time_game_selector: bool,
    pub games_played: i32,
    pub board_result: BoardResult,
    pub saw_yeti: bool,
    pub easy_planting_cheat: bool,
    pub crazy_dave_state: CrazyDaveState,
    pub crazy_dave_blink_counter: i32,
    pub crazy_dave_message_index: i32,
    pub crazy_dave_message_text: String,
    pub app_rand_seed: i32,
    pub play_time_active_session: i32,
    pub play_time_inactive_session: i32,
    pub mustache_mode: bool,
    pub super_mower_mode: bool,
    pub future_mode: bool,
    pub pinata_mode: bool,
    pub dance_mode: bool,
    pub daisy_mode: bool,
    pub tod_cheat_keys: bool,
    pub mute_sounds_for_cutscene: bool,
    pub trial_type: TrialType,
    pub debug_trial_locked: bool,
    pub konami_check: TypingCheck,
    pub mustache_check: TypingCheck,
    pub moustache_check: TypingCheck,
    pub super_mower_check: TypingCheck,
    pub super_mower_check2: TypingCheck,
    pub future_check: TypingCheck,
    pub pinata_check: TypingCheck,
    pub dance_check: TypingCheck,
    pub daisy_check: TypingCheck,
    pub sukhbir_check: TypingCheck,
    pub sukhbir_mode: bool,

    // SDL2 / 窗口相关
    pub screen_width: i32,
    pub screen_height: i32,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub is_down: bool,
}

impl LawnApp {
    pub fn new() -> Self {
        Self {
            board: None,
            title_screen: TitleScreen::new(),
            game_selector: GameSelector::new(),
            seed_chooser_screen: SeedChooserScreen::new(),
            award_screen: AwardScreen::new(),
            credit_screen: CreditScreen::new(),
            challenge_screen: ChallengeScreen::new(),
            effect_system: EffectSystem::new(),
            reanimator_cache: ReanimatorCache::new(),
            profile_mgr: ProfileMgr::new("profiles"),
            player_info: None,
            last_level_stats: LevelStats::new(),
            close_request: false,
            app_counter: 0,
            music: Music::new(),
            pool_effect: PoolEffect::new(),
            zen_garden: ZenGarden::new(GardenType::Main),
            game_mode: GameMode::Adventure,
            game_scene: GameScenes::Loading,
            loading_completed: false,
            first_time_game_selector: true,
            games_played: 0,
            board_result: BoardResult::None,
            saw_yeti: false,
            easy_planting_cheat: false,
            crazy_dave_state: CrazyDaveState::Off,
            crazy_dave_blink_counter: 0,
            crazy_dave_message_index: 0,
            crazy_dave_message_text: String::new(),
            app_rand_seed: 0,
            play_time_active_session: 0,
            play_time_inactive_session: 0,
            mustache_mode: false,
            super_mower_mode: false,
            future_mode: false,
            pinata_mode: false,
            dance_mode: false,
            daisy_mode: false,
            tod_cheat_keys: true,
            mute_sounds_for_cutscene: false,
            trial_type: TrialType::None,
            debug_trial_locked: false,
            konami_check: TypingCheck::new("upupdowndownleftrightleftrightba"),
            mustache_check: TypingCheck::new("mustache"),
            moustache_check: TypingCheck::new("moustache"),
            super_mower_check: TypingCheck::new("supermower"),
            super_mower_check2: TypingCheck::new("supermower"),
            future_check: TypingCheck::new("future"),
            pinata_check: TypingCheck::new("pinata"),
            dance_check: TypingCheck::new("dance"),
            daisy_check: TypingCheck::new("daisies"),
            sukhbir_check: TypingCheck::new("sukhbir"),
            sukhbir_mode: false,

            screen_width: 800,
            screen_height: 600,
            mouse_x: 0,
            mouse_y: 0,
            is_down: false,
        }
    }

    pub fn set_screen_size(&mut self, w: i32, h: i32) {
        self.screen_width = w;
        self.screen_height = h;
    }

    pub fn init(&mut self) {
        self.game_scene = GameScenes::Menu;
        log::info!("LawnApp initialized (screen: {}x{})", self.screen_width, self.screen_height);
    }

    pub fn start(&mut self) {
        self.game_scene = GameScenes::Menu;
        self.title_screen.start();
    }

    pub fn update(&mut self) {
        self.app_counter = self.app_counter.wrapping_add(1);
        match self.game_scene {
            GameScenes::Menu => {
                self.title_screen.update();
            }
            GameScenes::Playing => {
                if let Some(ref mut board) = self.board {
                    board.update();
                }
            }
            _ => {}
        }
        self.music.update();
        self.pool_effect.update();
        self.zen_garden.update();
        self.effect_system.update();
    }

    pub fn new_game(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
        self.make_new_board();
        self.start_playing();
    }

    pub fn make_new_board(&mut self) {
        self.board = Some(Box::new(Board::new()));
    }

    pub fn start_playing(&mut self) {
        self.game_scene = GameScenes::Playing;
    }

    pub fn kill_board(&mut self) {
        self.board = None;
    }

    pub fn show_seed_chooser_screen(&mut self) {
        self.seed_chooser_screen.active = true;
    }

    pub fn show_award_screen(&mut self, award_type: AwardType) {
        self.award_screen.active = true;
        self.award_screen.award_type = award_type;
        self.award_screen.counter = 0;
    }

    pub fn do_new_options(&mut self) {
        // TODO: 打开选项对话框
    }

    pub fn handle_cheat_code(&mut self, key: i32) {
        let ch = key as u8 as char;
        if self.konami_check.add_char(ch) {
            // 科乐美作弊码激活
        }
        if self.mustache_check.add_char(ch) {
            self.mustache_mode = !self.mustache_mode;
        }
        if self.future_check.add_char(ch) {
            self.future_mode = true;
        }
        if self.pinata_check.add_char(ch) {
            self.pinata_mode = true;
        }
        if self.dance_check.add_char(ch) {
            self.dance_mode = true;
        }
        if self.daisy_check.add_char(ch) {
            self.daisy_mode = true;
        }
    }

    pub fn is_adventure_mode(&self) -> bool {
        self.game_mode == GameMode::Adventure
    }

    pub fn is_survival_mode(&self) -> bool {
        matches!(
            self.game_mode,
            GameMode::SurvivalNormalStage1
                | GameMode::SurvivalNormalStage2
                | GameMode::SurvivalNormalStage3
                | GameMode::SurvivalNormalStage4
                | GameMode::SurvivalNormalStage5
                | GameMode::SurvivalHardStage1
                | GameMode::SurvivalHardStage2
                | GameMode::SurvivalHardStage3
                | GameMode::SurvivalHardStage4
                | GameMode::SurvivalHardStage5
                | GameMode::SurvivalEndlessStage1
                | GameMode::SurvivalEndlessStage2
                | GameMode::SurvivalEndlessStage3
                | GameMode::SurvivalEndlessStage4
                | GameMode::SurvivalEndlessStage5
        )
    }

    pub fn shutdown(&mut self) {
        self.kill_board();
        log::info!("LawnApp shutdown");
    }

    // ── 渲染 ──
    pub fn draw(&mut self, g: &Graphics, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        match self.game_scene {
            GameScenes::Menu => {
                self.title_screen.draw(g, canvas);
            }
            GameScenes::Playing => {
                if let Some(ref mut board) = self.board {
                    board.draw(g, canvas);
                }
            }
            _ => {}
        }
    }

    // ── 输入事件处理 ──
    pub fn key_down(&mut self, key: i32) {
        // 检测作弊码
        if self.tod_cheat_keys {
            self.handle_cheat_code(key);
        }
        match self.game_scene {
            GameScenes::Menu => {
                self.title_screen.key_down(key);
            }
            GameScenes::Playing => {
                if let Some(ref mut board) = self.board {
                    board.key_down(key);
                }
            }
            _ => {}
        }
    }

    pub fn key_up(&mut self, key: i32) {
        match self.game_scene {
            GameScenes::Menu => {
                self.title_screen.key_up(key);
            }
            GameScenes::Playing => {
                if let Some(ref mut board) = self.board {
                    board.key_up(key);
                }
            }
            _ => {}
        }
    }

    pub fn mouse_down(&mut self, x: i32, y: i32, clicks: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.is_down = true;
        match self.game_scene {
            GameScenes::Menu => {
                self.title_screen.mouse_down(x, y, clicks);
            }
            GameScenes::Playing => {
                if let Some(ref mut board) = self.board {
                    board.mouse_down(x, y, clicks);
                }
            }
            _ => {}
        }
    }

    pub fn mouse_up(&mut self, x: i32, y: i32, clicks: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.is_down = false;
        match self.game_scene {
            GameScenes::Menu => {
                self.title_screen.mouse_up(x, y, clicks);
            }
            GameScenes::Playing => {
                if let Some(ref mut board) = self.board {
                    board.mouse_up(x, y, clicks);
                }
            }
            _ => {}
        }
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
        match self.game_scene {
            GameScenes::Menu => {
                self.title_screen.mouse_move(x, y);
            }
            GameScenes::Playing => {
                if let Some(ref mut board) = self.board {
                    board.mouse_move(x, y);
                }
            }
            _ => {}
        }
    }
}
