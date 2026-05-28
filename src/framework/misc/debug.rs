// 对应 C++ 中的 SexyAppFramework/misc/Debug.h 和 Debug.cpp

pub fn debug_printf(msg: &str) {
    log::debug!("debug_printf: {}", msg);
}
