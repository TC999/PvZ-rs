// 对应 C++ 中的 LawnCommon.h / LawnCommon.cpp
// 草坪通用工具函数

use crate::const_enums::*;
use crate::game_constants::*;

pub fn grid_to_pixel_x(col: i32, _row: i32) -> i32 {
    LAWN_XMIN + col * 80
}

pub fn grid_to_pixel_y(_col: i32, row: i32) -> i32 {
    LAWN_YMIN + row * 100
}

pub fn pixel_to_grid_x(x: i32, _y: i32) -> i32 {
    (x - LAWN_XMIN) / 80
}

pub fn pixel_to_grid_y(_x: i32, y: i32) -> i32 {
    (y - LAWN_YMIN) / 100
}

pub fn is_valid_cell(col: i32, row: i32) -> bool {
    col >= 0 && col < 9 && row >= 0 && row < 5
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
