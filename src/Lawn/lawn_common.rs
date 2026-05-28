// 对应 C++ 中的 LawnCommon.h / LawnCommon.cpp
// 草坪通用工具函数

use crate::const_enums::*;
use crate::game_constants::*;

pub fn grid_to_pixel_x(col: i32, _row: i32) -> i32 {
    let result = LAWN_XMIN + col * 80;
    log::trace!("lawn_common::grid_to_pixel_x: 列 {} -> 像素 x={}", col, result);
    result
}

pub fn grid_to_pixel_y(_col: i32, row: i32) -> i32 {
    let result = LAWN_YMIN + row * 100;
    log::trace!("lawn_common::grid_to_pixel_y: 行 {} -> 像素 y={}", row, result);
    result
}

pub fn pixel_to_grid_x(x: i32, _y: i32) -> i32 {
    let result = (x - LAWN_XMIN) / 80;
    log::trace!("lawn_common::pixel_to_grid_x: 像素 x={} -> 列 {}", x, result);
    result
}

pub fn pixel_to_grid_y(_x: i32, y: i32) -> i32 {
    let result = (y - LAWN_YMIN) / 100;
    log::trace!("lawn_common::pixel_to_grid_y: 像素 y={} -> 行 {}", y, result);
    result
}

pub fn is_valid_cell(col: i32, row: i32) -> bool {
    let result = col >= 0 && col < 9 && row >= 0 && row < 5;
    log::trace!("lawn_common::is_valid_cell: 检查单元格 ({}, {})，结果 {}", col, row, result);
    result
}

pub fn planting_reason_to_string(reason: PlantingReason) -> &'static str {
    match reason {
        PlantingReason::Ok => "OK",
        PlantingReason::NotHere => "不能种在这里",
        PlantingReason::OnlyOnGraves => "只能种在墓碑上",
        PlantingReason::OnlyInPool => "只能种在泳池中",
        PlantingReason::OnlyOnGround => "只能种在地上",
        PlantingReason::NeedsPot => "需要花盆",
        PlantingReason::NotOnArt => "不能种在这里",
        PlantingReason::NotPassedLine => "不能种在红线右侧",
        PlantingReason::NeedsUpgrade => "需要升级",
        PlantingReason::NotOnGrave => "不能种在墓碑上",
        PlantingReason::NotOnCrater => "不能种在弹坑上",
        PlantingReason::NotOnWater => "不能种在水上",
        PlantingReason::NeedsGround => "需要地面",
        PlantingReason::NeedsSleeping => "需要唤醒",
    }
}
