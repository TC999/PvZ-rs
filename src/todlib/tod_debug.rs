// 对应 C++ 中的 Sexy.TodLib/TodDebug.h 和 TodDebug.cpp
// 调试和日志工具

use log::{debug, error, info, warn};

// 对应 C++ 中的 TodTrace、TodLog 等
pub struct TodDebug;

impl TodDebug {
    pub fn trace(msg: &str) {
        debug!("{}", msg);
    }

    pub fn log(msg: &str) {
        info!("{}", msg);
    }

    pub fn error(msg: &str) {
        error!("{}", msg);
    }

    pub fn assert_fail(msg: &str) {
        panic!("Assertion failed: {}", msg);
    }
}
