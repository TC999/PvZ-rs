// 对应 C++ 中的 Board.h / Board.cpp - 游戏主棋盘

use crate::const_enums::*;

pub struct Board {
    pub board_result: BoardResult,
    pub game_mode: GameMode,
    pub width: i32,
    pub height: i32,
    // C++ 中的 Board 包含大量成员变量和方法
    // 这里仅保留骨架，具体逻辑需要在后续迭代中填充
    pub grid_items: Vec<Option<GridItemType>>,
    pub grid_square_types: Vec<GridSquareType>,
    pub num_columns: i32,
    pub num_rows: i32,
    pub cursor_type: CursorType,
    pub plant_cooldowns: Vec<i32>,
}

impl Board {
    pub fn new() -> Self {
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
        // TODO: 实现游戏主循环更新逻辑
    }

    pub fn draw(&self) {
        // TODO: 实现游戏绘制逻辑
    }

    pub fn reset(&mut self) {
        self.board_result = BoardResult::None;
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
