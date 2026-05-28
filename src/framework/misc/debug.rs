// 对应 C++ 中的 SexyAppFramework/misc/Debug.h 和 Debug.cpp

use log::debug;

pub fn debug_printf(msg: &str) {
    debug!("{}", msg);
}
