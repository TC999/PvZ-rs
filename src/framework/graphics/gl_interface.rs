// 对应 C++ 中的 GLInterface.h / GLInterface.cpp
// OpenGL 渲染接口
//
// ASSUMPTION: C++ GLInterface 封装了 OpenGL 上下文的创建和管理。
// Rust 版本中由 sdl2::video::Window 和 OpenGL 窗口管理替代。

/// GLInterface - OpenGL 接口 (对应 C++ Sexy::GLInterface)
pub struct GLInterface {
    pub initialized: bool,
}

impl GLInterface {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    pub fn init(&mut self) -> bool {
        self.initialized = true;
        true
    }
}

impl Default for GLInterface {
    fn default() -> Self { Self::new() }
}
