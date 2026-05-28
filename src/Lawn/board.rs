// 对应 C++ 中的 Board.h / Board.cpp - 游戏主棋盘

use crate::const_enums::*;
use crate::framework::graphics::graphics::Graphics;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Board {
    pub board_result: BoardResult,
    pub game_mode: GameMode,
    pub width: i32,
    pub height: i32,
    pub grid_items: Vec<Option<GridItemType>>,
    pub grid_square_types: Vec<GridSquareType>,
    pub num_columns: i32,
    pub num_rows: i32,
    pub cursor_type: CursorType,
    pub plant_cooldowns: Vec<i32>,
}

impl Board {
    pub fn new() -> Self {
        log::info!("Board::new: 创建新的游戏板");
        Self {
            board_result: BoardResult::None,
            game_mode: GameMode::Adventure,
            width: 800,
            height: 600,
            grid_items: Vec::new(),
            grid_square_types: Vec::new(),
            num_columns: 9,
            num_rows: 5,
            cursor_type: CursorType::Normal,
            plant_cooldowns: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        log::trace!("Board::update: 更新游戏板");
        // TODO: 实现游戏主循环更新逻辑
    }

    pub fn draw(&self, _g: &Graphics, _canvas: &mut Canvas<Window>) {
        log::trace!("Board::draw: 绘制游戏板");
        // TODO: 实现游戏绘制逻辑
    }

    pub fn reset(&mut self) {
        log::info!("Board::reset: 重置游戏板");
        self.board_result = BoardResult::None;
    }

    pub fn key_down(&mut self, key: i32) {
        log::debug!("Board::key_down: 按键按下 {}", key);
    }
    pub fn key_up(&mut self, key: i32) {
        log::debug!("Board::key_up: 按键释放 {}", key);
    }
    pub fn mouse_down(&mut self, x: i32, y: i32, clicks: i32) {
        log::debug!("Board::mouse_down: 鼠标按下 ({}, {}), 点击次数 {}", x, y, clicks);
    }
    pub fn mouse_up(&mut self, x: i32, y: i32, clicks: i32) {
        log::debug!("Board::mouse_up: 鼠标释放 ({}, {}), 点击次数 {}", x, y, clicks);
    }
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        log::trace!("Board::mouse_move: 鼠标移动 ({}, {})", x, y);
    }
}
