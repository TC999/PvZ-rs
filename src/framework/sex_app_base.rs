// 对应 C++ 中的 SexyAppBase.h / SexyAppBase.cpp
// 应用程序框架基类
//
// C++ 依赖: SDL2, OpenGL, WidgetManager, SoundManager, MusicInterface, ImageLib 等
// Rust 映射: sdl2 crate, OpenGL, 各 framework 子模块

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::framework::misc::ratio::Ratio;
use crate::framework::misc::common::ulong;

// ─── Widget Manager 前向声明 ───
use crate::framework::widget::widget_manager::WidgetManager;

// ─── 声音系统 ───
use crate::framework::sound::sound_manager::SoundManager;
use crate::framework::sound::music_interface::MusicInterface;

// ─── 图形系统 ───
use crate::framework::graphics::image::Image;
use crate::framework::graphics::memory_image::MemoryImage;
use crate::framework::graphics::gl_interface::GLInterface;

// ─── 资源管理 ───
use crate::framework::misc::resource_manager::ResourceManager;

// ─── 随机数 ───
use crate::framework::misc::mt_rand::MTRand;

// ── Cursor 类型枚举 (对应 C++ enum CURSOR_*) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CursorType {
    Pointer = 0,
    Hand = 1,
    Dragging = 2,
    Text = 3,
    CircleSlash = 4,
    SizeAll = 5,
    SizeNESW = 6,
    SizeNS = 7,
    SizeNWSE = 8,
    SizeWE = 9,
    Wait = 10,
    None = 11,
    Custom = 12,
}

// ── Widget 安全删除信息 ──
#[derive(Clone)]
pub struct WidgetSafeDeleteInfo {
    pub update_app_depth: i32,
    pub widget_index: usize,  // ASSUMPTION: Use index into WidgetManager instead of raw pointer
}

// ── 类型别名 ──
pub type WidgetSafeDeleteList = Vec<WidgetSafeDeleteInfo>;
pub type MemoryImageSet = Vec<*mut MemoryImage>;
pub type DialogMap = HashMap<i32, i32>;  // Widget index
pub type DialogList = Vec<i32>;
pub type StringVector = Vec<String>;
pub type StringStringMap = HashMap<String, String>;
pub type StringBoolMap = HashMap<String, bool>;
pub type StringIntMap = HashMap<String, i32>;
pub type StringDoubleMap = HashMap<String, f64>;
pub type StringStringVectorMap = HashMap<String, StringVector>;

// ── 演示/录制模式 ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoFlags {
    Normal = 0,
    Playing = 1,
    Recording = 2,
}

// ── SexyAppBase - 应用程序框架基类 ──
pub struct SexyAppBase {
    // 基本属性 (对应 C++)
    pub m_title: String,
    pub m_prod_name: String,
    pub m_product_version: String,
    pub m_company_name: String,
    pub m_full_screen: bool,
    pub m_is_windowed: bool,
    pub m_width: i32,
    pub m_height: i32,
    pub m_display_width: i32,
    pub m_display_height: i32,
    pub m_game_screen_width: i32,
    pub m_game_screen_height: i32,
    pub m_screen_bpp: i32,

    // 帧/时间
    pub m_update_count: u32,
    pub m_update_app_mutex_count: i32,
    pub m_last_timer_check: u32,
    pub m_timer_milliseconds: u32,
    pub m_timer_milliseconds_drift: i32,
    pub m_vsync_broken: bool,
    pub m_auto_enable_3d: bool,
    pub m_vsync_broken_test_iterations: i32,
    pub m_vsync_broken_test_updates: i32,
    pub m_wait_for_vsync: bool,
    pub m_loaded: bool,
    pub m_yielding: bool,
    pub m_paused: bool,
    pub m_no_defer: bool,
    pub m_loading: bool,
    pub m_loading_failed: bool,

    // 帧率
    pub m_milli_per_frame: u32,
    pub m_updates_per_frame: u32,
    pub m_frame_time: f64,
    pub m_fps_total_time: f64,
    pub m_fps_count: i32,
    pub m_fps: i32,
    pub m_phys_avg_loop_count: i32,
    pub m_sleep_reached: bool,
    pub m_smooth_rendering: bool,

    // 光标
    pub m_cursor_num: CursorType,
    pub m_custom_cursor_data: Option<Vec<u32>>,
    pub m_custom_cursor_width: i32,
    pub m_custom_cursor_height: i32,
    pub m_custom_cursor_hot_x: i32,
    pub m_custom_cursor_hot_y: i32,

    // Widget 系统
    pub m_widget_manager: Option<WidgetManager>,
    pub m_safe_delete_list: WidgetSafeDeleteList,
    pub m_widget_manager_initialized: bool,

    // 声音系统
    pub m_sound_manager: Option<SoundManager>,
    pub m_music_interface: Option<MusicInterface>,
    pub m_sound_manager_installed: bool,
    pub m_muted: bool,

    // 图形系统
    pub m_gl_interface: Option<GLInterface>,
    pub m_image_set: MemoryImageSet,

    // 对话框
    pub m_dialog_map: DialogMap,
    pub m_dialog_list: DialogList,

    // 资源管理
    pub m_resource_manager: Option<ResourceManager>,

    // 应用数据
    pub m_app_data_folder: String,
    pub m_resource_folder: String,

    // 演示/录制
    pub m_demo_flag: DemoFlags,
    pub m_demo_prefix: String,
    pub m_demo_file_name: String,

    // 属性
    pub m_properties_map: StringStringMap,
    pub m_properties_bool_map: StringBoolMap,
    pub m_properties_int_map: StringIntMap,
    pub m_properties_double_map: StringDoubleMap,
    pub m_properties_vector_map: StringStringVectorMap,

    // 退出控制
    pub m_shutdown: bool,
    pub m_exit_code: i32,

    // 随机数
    pub m_rand: MTRand,

    // 注册表模拟 (C++ 使用 Windows 注册表或模拟)
    pub m_registry_values: StringStringMap,
    pub m_registry_need_write: bool,

    // 渲染比率
    pub m_screen_ratio: Ratio,

    // 调试
    pub m_allow_network: bool,
    pub m_update_app_depth: i32,
    pub m_in_update: bool,
    pub m_reg_serial: u32,
    pub m_suppress_regular_sleep: bool,

    // 内存跟踪
    pub m_total_memory: u64,

    // build 信息
    pub m_build_num: i32,
    pub m_build_date: String,
}

impl SexyAppBase {
    pub fn new() -> Self {
        log::info!("SexyAppBase::new: 创建应用程序框架基类实例");
        Self {
            m_title: String::new(),
            m_prod_name: String::new(),
            m_product_version: String::from("1.0"),
            m_company_name: String::from("Community"),
            m_full_screen: false,
            m_is_windowed: true,
            m_width: 800,
            m_height: 600,
            m_display_width: 800,
            m_display_height: 600,
            m_game_screen_width: 800,
            m_game_screen_height: 600,
            m_screen_bpp: 32,

            m_update_count: 0,
            m_update_app_mutex_count: 0,
            m_last_timer_check: 0,
            m_timer_milliseconds: 0,
            m_timer_milliseconds_drift: 0,
            m_vsync_broken: false,
            m_auto_enable_3d: true,
            m_vsync_broken_test_iterations: 0,
            m_vsync_broken_test_updates: 0,
            m_wait_for_vsync: true,
            m_loaded: false,
            m_yielding: false,
            m_paused: false,
            m_no_defer: false,
            m_loading: true,
            m_loading_failed: false,

            m_milli_per_frame: 10,
            m_updates_per_frame: 1,
            m_frame_time: 0.0,
            m_fps_total_time: 0.0,
            m_fps_count: 0,
            m_fps: 0,
            m_phys_avg_loop_count: 0,
            m_sleep_reached: false,
            m_smooth_rendering: true,

            m_cursor_num: CursorType::Pointer,
            m_custom_cursor_data: None,
            m_custom_cursor_width: 0,
            m_custom_cursor_height: 0,
            m_custom_cursor_hot_x: 0,
            m_custom_cursor_hot_y: 0,

            m_widget_manager: None,
            m_safe_delete_list: Vec::new(),
            m_widget_manager_initialized: false,

            m_sound_manager: None,
            m_music_interface: None,
            m_sound_manager_installed: false,
            m_muted: false,

            m_gl_interface: None,
            m_image_set: Vec::new(),

            m_dialog_map: HashMap::new(),
            m_dialog_list: Vec::new(),

            m_resource_manager: None,

            m_app_data_folder: String::new(),
            m_resource_folder: String::new(),

            m_demo_flag: DemoFlags::Normal,
            m_demo_prefix: String::from("pvzp"),
            m_demo_file_name: String::from("pvzp.dmo"),

            m_properties_map: HashMap::new(),
            m_properties_bool_map: HashMap::new(),
            m_properties_int_map: HashMap::new(),
            m_properties_double_map: HashMap::new(),
            m_properties_vector_map: HashMap::new(),

            m_shutdown: false,
            m_exit_code: 0,

            m_rand: MTRand::new(),

            m_registry_values: HashMap::new(),
            m_registry_need_write: false,

            m_screen_ratio: Ratio::new(1, 1),

            m_allow_network: false,
            m_update_app_depth: 0,
            m_in_update: false,
            m_reg_serial: 0,
            m_suppress_regular_sleep: false,

            m_total_memory: 0,

            m_build_num: 0,
            m_build_date: String::new(),
        }
    }

    // ── 初始化方法 ──

    /// 初始化应用 (对应 C++ SexyAppBase::Init)
    pub fn init(&mut self) {
        log::info!("SexyAppBase::init: 初始化应用程序框架");
        // 设置随机种子
        let seed = Instant::now().elapsed().as_nanos() as u64;
        log::debug!("SexyAppBase::init: 设置随机种子 {}", seed);
        self.m_rand = MTRand::with_seed(seed);

        // 初始化 WidgetManager
        log::debug!("SexyAppBase::init: 初始化 WidgetManager");
        let mut wm = WidgetManager::new();
        wm.width = self.m_width;
        wm.height = self.m_height;
        self.m_widget_manager = Some(wm);
        self.m_widget_manager_initialized = true;

        // 设置资源目录
        self.set_resource_folder("");
        log::debug!("SexyAppBase::init: 设置资源目录为空");

        // 设置应用数据目录
        self.set_app_data_folder("userdata");
        log::debug!("SexyAppBase::init: 设置应用数据目录为 userdata");

        self.m_loaded = true;
        log::info!("SexyAppBase::init: 应用程序框架初始化完成");
    }

    /// 开始运行 (对应 C++ SexyAppBase::Start)
    pub fn start(&mut self) {
        log::info!("SexyAppBase::start: 启动应用程序");
        self.m_shutdown = false;
    }

    /// 更新帧 (对应 C++ SexyAppBase::UpdateFrames)
    pub fn update_frames(&mut self) {
        log::trace!("SexyAppBase::update_frames: 更新帧计数 {}", self.m_update_count);
        // 更新帧计数
        self.m_update_count += 1;

        // 更新时间
        let _ = Instant::now();

        // 更新 FPS
        self.m_fps_total_time += 0.016; // ~60 FPS
        self.m_fps_count += 1;
        if self.m_fps_total_time >= 1.0 {
            log::trace!("SexyAppBase::update_frames: FPS 更新为 {}", self.m_fps_count);
            self.m_fps = self.m_fps_count;
            self.m_fps_count = 0;
            self.m_fps_total_time = 0.0;
        }

        // 处理安全的 widget 删除
        self.process_safe_delete_list();
    }

    /// 主循环更新 (对应 C++ SexyAppBase::Update)
    pub fn update(&mut self) {
        log::trace!("SexyAppBase::update: 开始更新，shutdown={}", self.m_shutdown);
        if self.m_shutdown {
            log::trace!("SexyAppBase::update: 应用已关闭，跳过更新");
            return;
        }

        self.m_in_update = true;
        self.m_update_app_depth += 1;

        // 更新 WidgetManager
        if let Some(ref mut wm) = self.m_widget_manager {
            log::trace!("SexyAppBase::update: 更新 WidgetManager");
            wm.update();
        }

        // 更新音乐
        if let Some(ref _music) = self.m_music_interface {
            // Music update happens per-frame (FadeIn/FadeOut)
        }

        self.m_update_app_depth -= 1;
        self.m_in_update = false;

        self.process_safe_delete_list();
        log::trace!("SexyAppBase::update: 更新完成");
    }

    /// 关闭 (对应 C++ SexyAppBase::Shutdown)
    pub fn shutdown(&mut self) {
        log::info!("SexyAppBase::shutdown: 关闭应用程序");
        self.m_shutdown = true;
    }

    /// 退出 (对应 C++ SexyAppBase::DoExit)
    pub fn do_exit(&mut self, code: i32) {
        log::info!("SexyAppBase::do_exit: 退出应用程序，退出码 {}", code);
        self.m_exit_code = code;
        self.m_shutdown = true;
    }

    // ── Widget 管理 ──

    /// 添加 Widget (对应 C++ SexyAppBase::AddWidget)
    // ASSUMPTION: C++ uses raw Widget pointers; Rust uses Box<dyn WidgetLike>.
    // Caller should use widget_manager directly for Box-based widgets.
    pub fn add_widget_index(&mut self, _widget_index: usize) {
        // Widgets are managed by WidgetManager via Box<dyn WidgetLike>
        // This method is for C++ compatibility; actual widgets are added via WidgetManager
    }

    /// 安全删除 Widget (对应 C++ SexyAppBase::SafeDeleteWidget)
    pub fn safe_delete_widget(&mut self, widget_index: usize) {
        let info = WidgetSafeDeleteInfo {
            update_app_depth: self.m_update_app_depth,
            widget_index,
        };
        self.m_safe_delete_list.push(info);
    }

    /// 处理安全删除列表 (对应 C++ SexyAppBase::ProcessSafeDeleteList)
    fn process_safe_delete_list(&mut self) {
        let current_depth = self.m_update_app_depth;
        self.m_safe_delete_list.retain(|info| {
            !(info.update_app_depth <= current_depth)
            // ASSUMPTION: In C++ the widget is deleted via raw pointer.
            // In Rust, widgets are owned by WidgetManager via Box.
            // Deletion happens when the Box is removed from the Vec.
        });
    }

    // ── 对话框管理 ──

    /// 添加对话框 (对应 C++ SexyAppBase::AddDialog)
    pub fn add_dialog(&mut self, dialog_id: i32, widget_index: i32) {
        log::info!("SexyAppBase::add_dialog: 添加对话框，ID {}，控件索引 {}", dialog_id, widget_index);
        self.m_dialog_map.insert(dialog_id, widget_index);
        self.m_dialog_list.push(widget_index);
    }

    /// 对话框是否激活 (对应 C++ SexyAppBase::IsDialogActive)
    pub fn is_dialog_active(&self, dialog_id: i32) -> bool {
        let result = self.m_dialog_map.contains_key(&dialog_id);
        log::trace!("SexyAppBase::is_dialog_active: 检查对话框 {} 是否激活，结果 {}", dialog_id, result);
        result
    }

    /// 关闭对话框 (对应 C++ SexyAppBase::KillDialog)
    pub fn kill_dialog(&mut self, dialog_id: i32) {
        log::info!("SexyAppBase::kill_dialog: 关闭对话框 {}", dialog_id);
        if let Some(index) = self.m_dialog_map.remove(&dialog_id) {
            self.m_dialog_list.retain(|&i| i != index);
        }
    }

    /// 关闭所有对话框 (对应 C++ SexyAppBase::KillAllDialogs)
    pub fn kill_all_dialogs(&mut self) {
        log::info!("SexyAppBase::kill_all_dialogs: 关闭所有对话框，数量 {}", self.m_dialog_map.len());
        self.m_dialog_map.clear();
        self.m_dialog_list.clear();
    }

    // ── 属性/配置管理 ──

    /// 读取注册表 (对应 C++ SexyAppBase::ReadFromRegistry)
    pub fn read_from_registry(&mut self) {
        log::info!("SexyAppBase::read_from_registry: 读取注册表");
        let reg_path = self.get_registry_path();
        if let Ok(data) = fs::read_to_string(&reg_path) {
            for line in data.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    self.m_registry_values.insert(key.to_string(), value.to_string());
                }
            }
            log::debug!("SexyAppBase::read_from_registry: 读取了 {} 个注册表值", self.m_registry_values.len());
        } else {
            log::warn!("SexyAppBase::read_from_registry: 注册表文件读取失败");
        }

        // 恢复保存的设置
        self.m_is_windowed = self.get_registry_bool("IsWindowed", true);
        self.m_full_screen = !self.m_is_windowed;
        log::debug!("SexyAppBase::read_from_registry: 窗口模式 {}", self.m_is_windowed);
    }

    /// 写入注册表 (对应 C++ SexyAppBase::WriteToRegistry)
    pub fn write_to_registry(&mut self) {
        log::info!("SexyAppBase::write_to_registry: 写入注册表");
        if !self.m_registry_need_write {
            log::trace!("SexyAppBase::write_to_registry: 无需写入");
            return;
        }

        let reg_path = self.get_registry_path();
        let mut content = String::new();
        for (key, value) in &self.m_registry_values {
            content.push_str(&format!("{}={}\n", key, value));
        }
        let _ = fs::write(&reg_path, &content);
        self.m_registry_need_write = false;
        log::info!("SexyAppBase::write_to_registry: 注册表写入完成");
    }

    /// 获取注册表值
    fn get_registry_bool(&self, key: &str, default: bool) -> bool {
        self.m_registry_values.get(key)
            .map(|v| v == "true" || v == "1")
            .unwrap_or(default)
    }

    /// 设置注册表值
    pub fn set_registry_value(&mut self, key: &str, value: &str) {
        log::debug!("SexyAppBase::set_registry_value: 设置注册表值 {}={}", key, value);
        self.m_registry_values.insert(key.to_string(), value.to_string());
        self.m_registry_need_write = true;
    }

    fn get_registry_path(&self) -> String {
        format!("{}/registry.txt", self.m_app_data_folder)
    }

    // ── 命令行参数处理 ──

    pub fn handle_cmd_line_param(&mut self, _param_name: &str, _param_value: &str) {
        // 基类默认空处理，子类 (SexyApp) 覆写
    }

    // ── 属性文件加载 ──

    /// 加载属性文件 (对应 C++ SexyAppBase::LoadProperties)
    pub fn load_properties(&mut self, filename: &str, _check_sig: bool) -> bool {
        log::info!("SexyAppBase::load_properties: 加载属性文件 {}", filename);
        // 简化实现：从 XML 或 INI 文件加载属性
        match fs::read_to_string(filename) {
            Ok(content) => {
                // 简单键值对解析
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                        continue;
                    }
                    if let Some((key, value)) = line.split_once('=') {
                        let key = key.trim().to_string();
                        let value = value.trim().to_string();
                        self.m_properties_map.insert(key, value);
                    }
                }
                log::info!("SexyAppBase::load_properties: 属性文件 {} 加载完成，共 {} 个属性", filename, self.m_properties_map.len());
                true
            }
            Err(e) => {
                log::error!("SexyAppBase::load_properties: 属性文件 {} 加载失败: {}", filename, e);
                false
            }
        }
    }

    /// 获取字符串属性 (对应 C++ SexyAppBase::GetString)
    pub fn get_string(&self, key: &str, default: &str) -> String {
        self.m_properties_map.get(key).cloned().unwrap_or_else(|| default.to_string())
    }

    /// 获取布尔属性 (对应 C++ SexyAppBase::GetBoolean)
    pub fn get_boolean(&self, key: &str, default: bool) -> bool {
        self.m_properties_map.get(key)
            .map(|v| v == "true" || v == "1" || v == "yes")
            .unwrap_or(default)
    }

    /// 获取整数属性 (对应 C++ SexyAppBase::GetInteger)
    pub fn get_integer(&self, key: &str, default: i32) -> i32 {
        self.m_properties_map.get(key)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// 获取浮点属性 (对应 C++ SexyAppBase::GetDouble)
    pub fn get_double(&self, key: &str, default: f64) -> f64 {
        self.m_properties_map.get(key)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    // ── 资源路径管理 ──

    pub fn set_app_data_folder(&mut self, path: &str) {
        log::info!("SexyAppBase::set_app_data_folder: 设置应用数据文件夹 {}", path);
        self.m_app_data_folder = path.to_string();
        // 确保目录存在
        let _ = fs::create_dir_all(path);
    }

    pub fn get_app_data_folder(&self) -> &str {
        &self.m_app_data_folder
    }

    pub fn set_resource_folder(&mut self, path: &str) {
        log::info!("SexyAppBase::set_resource_folder: 设置资源文件夹 {}", path);
        self.m_resource_folder = path.to_string();
    }

    pub fn get_resource_folder(&self) -> &str {
        &self.m_resource_folder
    }

    // ── 声音管理 ──

    pub fn install_sound_manager(&mut self, manager: SoundManager) {
        log::info!("SexyAppBase::install_sound_manager: 安装音效管理器");
        self.m_sound_manager = Some(manager);
        self.m_sound_manager_installed = true;
    }

    pub fn set_music_interface(&mut self, music: MusicInterface) {
        log::info!("SexyAppBase::set_music_interface: 设置音乐接口");
        self.m_music_interface = Some(music);
    }

    pub fn mute(&mut self, muted: bool) {
        log::info!("SexyAppBase::mute: 设置静音 {}", muted);
        self.m_muted = muted;
        if let Some(ref mut sm) = self.m_sound_manager {
            sm.set_master_volume(if muted { 0.0 } else { 1.0 });
        }
    }

    // ── Cursor 管理 ──

    pub fn set_cursor(&mut self, cursor: CursorType) {
        log::debug!("SexyAppBase::set_cursor: 设置光标 {:?}", cursor);
        self.m_cursor_num = cursor;
    }

    /// 隐藏/显示光标 (对应 C++ SexyAppBase::SetCursorImage)
    pub fn set_cursor_image(&mut self, cursor: CursorType, _image: Option<&Image>) {
        log::debug!("SexyAppBase::set_cursor_image: 设置光标图像 {:?}", cursor);
        self.m_cursor_num = cursor;
    }

    // ── 帧率控制 ──

    /// 获取每帧毫秒数 (对应 C++ SexyAppBase::GetMilliPerFrame)
    pub fn get_milli_per_frame(&self) -> u32 {
        log::trace!("SexyAppBase::get_milli_per_frame: 获取每帧毫秒数 {}", self.m_milli_per_frame);
        self.m_milli_per_frame
    }

    /// 设置帧率 (对应 C++ SexyAppBase::SetFramesPerSecond)
    pub fn set_frames_per_second(&mut self, fps: u32) {
        log::info!("SexyAppBase::set_frames_per_second: 设置帧率 {}", fps);
        if fps > 0 {
            self.m_milli_per_frame = 1000 / fps;
        }
    }

    // ── 异常/错误处理 ──

    pub fn get_game_seh_info(&self) -> String {
        format!(
            "Build Num: {}\r\nBuild Date: {}\r\n",
            self.m_build_num, self.m_build_date
        )
    }

    // ── 屏幕保护程序检测 ──

    pub fn is_screen_saver(&self) -> bool {
        false  // ASSUMPTION: non-Windows platforms no screen saver check
    }

    // ── 内存图像管理 ──

    pub fn add_memory_image(&mut self, image: *mut MemoryImage) {
        self.m_image_set.push(image);
    }

    pub fn remove_memory_image(&mut self, image: *mut MemoryImage) {
        self.m_image_set.retain(|&img| img != image);
    }

    // ── 工具方法 ──

    /// 检查文件是否存在 (对应 C++ FileExists)
    pub fn file_exists(filename: &str) -> bool {
        let result = Path::new(filename).exists();
        log::trace!("SexyAppBase::file_exists: 检查文件 {} 是否存在，结果 {}", filename, result);
        result
    }

    /// 获取随机种子 (对应 C++ SexyAppBase::SRand)
    pub fn srand(&mut self, seed: u64) {
        log::info!("SexyAppBase::srand: 设置随机种子 {}", seed);
        self.m_rand = MTRand::with_seed(seed);
    }

    /// 获取随机数 (对应 C++ Rand)
    pub fn rand(&mut self) -> i32 {
        let result = self.m_rand.next_int(i32::MAX);
        log::trace!("SexyAppBase::rand: 生成随机数 {}", result);
        result
    }

    pub fn rand_range(&mut self, range: i32) -> i32 {
        let result = self.m_rand.next_int(range);
        log::trace!("SexyAppBase::rand_range: 生成随机数 {}，范围 [0, {})", result, range);
        result
    }
}
