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
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    app.shutdown();
    log::info!("PvZ-Portable shutting down.");
}
