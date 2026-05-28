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
        log::info!("LawnApp::new: 创建新的LawnApp实例");
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
        log::debug!("LawnApp::set_screen_size: 设置屏幕尺寸为 {}x{}", w, h);
        self.screen_width = w;
        self.screen_height = h;
    }

    pub fn init(&mut self) {
        log::info!("LawnApp::init: 初始化LawnApp，屏幕尺寸 {}x{}", self.screen_width, self.screen_height);
        self.game_scene = GameScenes::Menu;
        log::info!("LawnApp::init: 游戏场景设置为 Menu");
    }

    pub fn start(&mut self) {
        log::info!("LawnApp::start: 启动游戏，设置场景为 Menu");
        self.game_scene = GameScenes::Menu;
        self.title_screen.start();
        log::info!("LawnApp::start: 标题屏幕已启动");
    }

    pub fn update(&mut self) {
        log::trace!("LawnApp::update: 更新计数器 {}", self.app_counter);
        self.app_counter = self.app_counter.wrapping_add(1);
        match self.game_scene {
            GameScenes::Menu => {
                log::trace!("LawnApp::update: 当前场景 Menu，更新标题屏幕");
                self.title_screen.update();
            }
            GameScenes::Playing => {
                log::trace!("LawnApp::update: 当前场景 Playing，更新游戏板");
                if let Some(ref mut board) = self.board {
                    board.update();
                }
            }
            _ => {
                log::trace!("LawnApp::update: 当前场景 {:?}", self.game_scene);
            }
        }
        self.music.update();
        self.pool_effect.update();
        self.zen_garden.update();
        self.effect_system.update();
    }

    pub fn new_game(&mut self, game_mode: GameMode) {
        log::info!("LawnApp::new_game: 开始新游戏，模式 {:?}", game_mode);
        self.game_mode = game_mode;
        self.make_new_board();
        self.start_playing();
        log::info!("LawnApp::new_game: 新游戏初始化完成");
    }

    pub fn make_new_board(&mut self) {
        log::info!("LawnApp::make_new_board: 创建新的游戏板");
        self.board = Some(Box::new(Board::new()));
        log::info!("LawnApp::make_new_board: 游戏板创建完成");
    }

    pub fn start_playing(&mut self) {
        log::info!("LawnApp::start_playing: 开始游戏，设置场景为 Playing");
        self.game_scene = GameScenes::Playing;
    }

    pub fn kill_board(&mut self) {
        log::info!("LawnApp::kill_board: 销毁游戏板");
        self.board = None;
    }

    pub fn show_seed_chooser_screen(&mut self) {
        log::info!("LawnApp::show_seed_chooser_screen: 显示种子选择屏幕");
        self.seed_chooser_screen.active = true;
    }

    pub fn show_award_screen(&mut self, award_type: AwardType) {
        log::info!("LawnApp::show_award_screen: 显示奖励屏幕，类型 {:?}", award_type);
        self.award_screen.active = true;
        self.award_screen.award_type = award_type;
        self.award_screen.counter = 0;
    }

    pub fn do_new_options(&mut self) {
        log::info!("LawnApp::do_new_options: 打开选项对话框（待实现）");
        // TODO: 打开选项对话框
    }

    pub fn handle_cheat_code(&mut self, key: i32) {
        log::debug!("LawnApp::handle_cheat_code: 处理按键 {}", key);
        let ch = key as u8 as char;
        if self.konami_check.add_char(ch) {
            log::info!("LawnApp::handle_cheat_code: 科乐美作弊码激活");
        }
        if self.mustache_check.add_char(ch) {
            self.mustache_mode = !self.mustache_mode;
            log::info!("LawnApp::handle_cheat_code: 胡子模式 {}", self.mustache_mode);
        }
        if self.future_check.add_char(ch) {
            self.future_mode = true;
            log::info!("LawnApp::handle_cheat_code: 未来模式激活");
        }
        if self.pinata_check.add_char(ch) {
            self.pinata_mode = true;
            log::info!("LawnApp::handle_cheat_code: 皮纳塔模式激活");
        }
        if self.dance_check.add_char(ch) {
            self.dance_mode = true;
            log::info!("LawnApp::handle_cheat_code: 舞蹈模式激活");
        }
        if self.daisy_check.add_char(ch) {
            self.daisy_mode = true;
            log::info!("LawnApp::handle_cheat_code: 雏菊模式激活");
        }
    }

    pub fn is_adventure_mode(&self) -> bool {
        let result = self.game_mode == GameMode::Adventure;
        log::trace!("LawnApp::is_adventure_mode: {}", result);
        result
    }

    pub fn is_survival_mode(&self) -> bool {
        let result = matches!(
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
        );
        log::trace!("LawnApp::is_survival_mode: {}", result);
        result
    }

    pub fn shutdown(&mut self) {
        log::info!("LawnApp::shutdown: 关闭LawnApp");
        self.kill_board();
        log::info!("LawnApp::shutdown: LawnApp已关闭");
    }

    // ── 渲染 ──
    pub fn draw(&mut self, g: &Graphics, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        log::trace!("LawnApp::draw: 开始绘制，当前场景 {:?}", self.game_scene);
        match self.game_scene {
            GameScenes::Menu => {
                log::trace!("LawnApp::draw: 绘制标题屏幕");
                self.title_screen.draw(g, canvas);
            }
            GameScenes::Playing => {
                log::trace!("LawnApp::draw: 绘制游戏板");
                if let Some(ref mut board) = self.board {
                    board.draw(g, canvas);
                }
            }
            _ => {
                log::trace!("LawnApp::draw: 未知场景 {:?}", self.game_scene);
            }
        }
    }

    // ── 输入事件处理 ──
    pub fn key_down(&mut self, key: i32) {
        log::debug!("LawnApp::key_down: 按键按下 {}", key);
        // 检测作弊码
        if self.tod_cheat_keys {
            self.handle_cheat_code(key);
        }
        match self.game_scene {
            GameScenes::Menu => {
                log::trace!("LawnApp::key_down: 转发按键到标题屏幕");
                self.title_screen.key_down(key);
            }
            GameScenes::Playing => {
                log::trace!("LawnApp::key_down: 转发按键到游戏板");
                if let Some(ref mut board) = self.board {
                    board.key_down(key);
                }
            }
            _ => {
                log::trace!("LawnApp::key_down: 未知场景 {:?}", self.game_scene);
            }
        }
    }

    pub fn key_up(&mut self, key: i32) {
        log::debug!("LawnApp::key_up: 按键释放 {}", key);
        match self.game_scene {
            GameScenes::Menu => {
                log::trace!("LawnApp::key_up: 转发按键到标题屏幕");
                self.title_screen.key_up(key);
            }
            GameScenes::Playing => {
                log::trace!("LawnApp::key_up: 转发按键到游戏板");
                if let Some(ref mut board) = self.board {
                    board.key_up(key);
                }
            }
            _ => {
                log::trace!("LawnApp::key_up: 未知场景 {:?}", self.game_scene);
            }
        }
    }

    pub fn mouse_down(&mut self, x: i32, y: i32, clicks: i32) {
        log::debug!("LawnApp::mouse_down: 鼠标按下 ({}, {}), 点击次数 {}", x, y, clicks);
        self.mouse_x = x;
        self.mouse_y = y;
        self.is_down = true;
        match self.game_scene {
            GameScenes::Menu => {
                log::trace!("LawnApp::mouse_down: 转发鼠标事件到标题屏幕");
                self.title_screen.mouse_down(x, y, clicks);
            }
            GameScenes::Playing => {
                log::trace!("LawnApp::mouse_down: 转发鼠标事件到游戏板");
                if let Some(ref mut board) = self.board {
                    board.mouse_down(x, y, clicks);
                }
            }
            _ => {
                log::trace!("LawnApp::mouse_down: 未知场景 {:?}", self.game_scene);
            }
        }
    }

    pub fn mouse_up(&mut self, x: i32, y: i32, clicks: i32) {
        log::debug!("LawnApp::mouse_up: 鼠标释放 ({}, {}), 点击次数 {}", x, y, clicks);
        self.mouse_x = x;
        self.mouse_y = y;
        self.is_down = false;
        match self.game_scene {
            GameScenes::Menu => {
                log::trace!("LawnApp::mouse_up: 转发鼠标事件到标题屏幕");
                self.title_screen.mouse_up(x, y, clicks);
            }
            GameScenes::Playing => {
                log::trace!("LawnApp::mouse_up: 转发鼠标事件到游戏板");
                if let Some(ref mut board) = self.board {
                    board.mouse_up(x, y, clicks);
                }
            }
            _ => {
                log::trace!("LawnApp::mouse_up: 未知场景 {:?}", self.game_scene);
            }
        }
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        log::trace!("LawnApp::mouse_move: 鼠标移动 ({}, {})", x, y);
        self.mouse_x = x;
        self.mouse_y = y;
        match self.game_scene {
            GameScenes::Menu => {
                log::trace!("LawnApp::mouse_move: 转发鼠标事件到标题屏幕");
                self.title_screen.mouse_move(x, y);
            }
            GameScenes::Playing => {
                log::trace!("LawnApp::mouse_move: 转发鼠标事件到游戏板");
                if let Some(ref mut board) = self.board {
                    board.mouse_move(x, y);
                }
            }
            _ => {
                log::trace!("LawnApp::mouse_move: 未知场景 {:?}", self.game_scene);
            }
        }
    }
}
