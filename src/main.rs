// PvZ-Portable Rust rewrite - main entry point
// 对应 C++ 中的 main.cpp

use pvz_portable::LawnApp;

fn main() {
    env_logger::init();
    log::info!("PvZ-Portable starting up...");

    let mut app = LawnApp::new();
    app.init();
    app.start();

    // 主游戏循环
    // TODO: 初始化 SDL2 窗口，运行事件循环
    while !app.close_request {
        app.update();
        // 约 60fps 的帧率控制
        // std::thread::sleep(std::time::Duration::from_millis(16));
        // FIXME: 实际主循环需要 SDL2 事件处理和 OpenGL 渲染
        break; // 目前没有实际渲染，立即退出
    }

    app.shutdown();
    log::info!("PvZ-Portable shutting down.");
}
