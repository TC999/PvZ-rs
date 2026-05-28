// PvZ-Portable Rust rewrite - main entry point
// 初始化 SDL2 视频、创建窗口和渲染器、运行游戏主循环

use pvz_portable::LawnApp;
use pvz_portable::framework::graphics::graphics::Graphics;

fn main() -> Result<(), String> {
    env_logger::init();
    log::info!("PvZ-Portable starting up...");

    // ── SDL2 初始化 ──
    let sdl_context = sdl2::init().map_err(|e| format!("SDL2 init failed: {}", e))?;
    let video_subsystem = sdl_context.video().map_err(|e| format!("SDL2 video init failed: {}", e))?;

    // ── 创建窗口 ──
    let window = video_subsystem
        .window("PvZ Portable", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| format!("Window creation failed: {}", e))?;

    // ── 创建 SDL2 渲染器 (对应 C++ 中的 GLInterface / 软件渲染) ──
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| format!("Canvas creation failed: {}", e))?;

    // ── 创建绘图上下文 ──
    // sdl_canvas raw pointer reserved for future FFI use

    // ── 创建 LawnApp ──
    let (win_w, win_h) = canvas.window().size();
    let mut app = LawnApp::new();
    app.set_screen_size(win_w as i32, win_h as i32);
    app.init();

    // ── 事件泵 ──
    let mut event_pump = sdl_context
        .event_pump()
        .map_err(|e| format!("Event pump creation failed: {}", e))?;

    let frame_duration = std::time::Duration::from_millis(16);

    // ── 主游戏循环 ──
    let mut last_time = std::time::Instant::now();
    'running: loop {
        if app.close_request {
            break 'running;
        }

        // 处理 SDL 事件
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::Quit { .. } => {
                    app.close_request = true;
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    app.close_request = true;
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    app.key_down(unsafe { std::mem::transmute::<sdl2::keyboard::Keycode, i32>(key) });
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    app.key_up(unsafe { std::mem::transmute::<sdl2::keyboard::Keycode, i32>(key) });
                }
                Event::MouseButtonDown {
                    x, y, clicks, ..
                } => {
                    app.mouse_down(x, y, clicks as i32);
                }
                Event::MouseButtonUp {
                    x, y, clicks, ..
                } => {
                    app.mouse_up(x, y, clicks as i32);
                }
                Event::MouseMotion { x, y, .. } => {
                    app.mouse_move(x, y);
                }
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    app.set_screen_size(w, h);
                }
                _ => {}
            }
        }

        let now = std::time::Instant::now();
        last_time = now;

        // 更新逻辑
        app.update();

        // 渲染
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // 绘制
        let g = Graphics::new(app.screen_width, app.screen_height);
        app.draw(&g, &mut canvas);

        canvas.present();

        // 帧率控制
        if let Some(sleep_time) = frame_duration.checked_sub(last_time.elapsed()) {
            std::thread::sleep(sleep_time);
        }
    }

    app.shutdown();
    log::info!("PvZ-Portable shutting down.");
    Ok(())
}
