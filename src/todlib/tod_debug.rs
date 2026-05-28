// 对应 C++ 中的 Sexy.TodLib/TodDebug.h 和 TodDebug.cpp
// 调试和日志工具

use log::{debug, error, info, warn};

// 对应 C++ 中的 TodTrace、TodLog 等
pub struct TodDebug;

impl TodDebug {
    pub fn trace(msg: &str) {
        log::trace!("TodDebug::trace: {}", msg);
    }

    pub fn log(msg: &str) {
        log::info!("TodDebug::log: {}", msg);
    }

    pub fn error(msg: &str) {
        log::error!("TodDebug::error: {}", msg);
    }

    pub fn assert_fail(msg: &str) {
        log::error!("TodDebug::assert_fail: 断言失败: {}", msg);
        panic!("Assertion failed: {}", msg);
    }
}
