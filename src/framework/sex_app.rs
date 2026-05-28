// 对应 C++ 中的 SexyApp.h / SexyApp.cpp
// 应用程序主类 (继承 SexyAppBase)

use crate::framework::sex_app_base::SexyAppBase;

/// SexyApp - 应用程序主类 (对应 C++ Sexy::SexyApp)
pub struct SexyApp {
    pub base: SexyAppBase,
    pub m_build_num: i32,
    pub m_build_date: String,
}

impl SexyApp {
    pub fn new() -> Self {
        log::info!("SexyApp::new: 创建应用程序主类");
        let mut app = Self {
            base: SexyAppBase::new(),
            m_build_num: 0,
            m_build_date: String::new(),
        };
        app.base.m_build_num = 0;
        app
    }

    /// 初始化
    pub fn init(&mut self) {
        log::info!("SexyApp::init: 初始化应用程序");
        log::info!("Product: {}", self.base.m_prod_name);
        log::info!("BuildNum: {}", self.m_build_num);
        log::info!("BuildDate: {}", self.m_build_date);

        self.base.init();
        log::info!("SexyApp::init: 应用程序初始化完成");
    }

    /// 初始化属性钩子 (对应 C++ SexyApp::InitPropertiesHook)
    pub fn init_properties_hook(&mut self) {
        log::info!("SexyApp::init_properties_hook: 初始化属性钩子");
        // 检查是否屏幕保护程序模式
        let check_sig = !self.base.is_screen_saver();
        log::debug!("SexyApp::init_properties_hook: 检查签名 {}", check_sig);

        // 加载 partner 属性
        log::debug!("SexyApp::init_properties_hook: 加载 partner 属性");
        self.base.load_properties("properties/partner.xml", check_sig);

        // 从属性中获取产品名
        self.base.m_prod_name = self.base.get_string("ProdName", &self.base.m_prod_name);
        log::debug!("SexyApp::init_properties_hook: 产品名 {}", self.base.m_prod_name);

        // 加载默认窗口模式
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            self.base.m_is_windowed = self.base.get_boolean("DefaultWindowed", self.base.m_is_windowed);
            log::debug!("SexyApp::init_properties_hook: 窗口模式 {}", self.base.m_is_windowed);
        }

        // 更新标题
        let new_title = self.base.get_string("Title", "");
        if !new_title.is_empty() {
            self.base.m_title = format!("{} {}", new_title, self.base.m_product_version);
            log::debug!("SexyApp::init_properties_hook: 标题 {}", self.base.m_title);
        }
        log::info!("SexyApp::init_properties_hook: 属性钩子初始化完成");
    }

    /// 处理命令行参数 (对应 C++ SexyApp::HandleCmdLineParam)
    pub fn handle_cmd_line_param(&mut self, param_name: &str, _param_value: &str) {
        log::info!("SexyApp::handle_cmd_line_param: 处理命令行参数 {}", param_name);
        match param_name {
            "-version" => {
                let version_string = format!(
                    "Product: {}\nVersion: {}\nBuild Num: {}\nBuild Date: {}\nLicense: LGPL-3.0-or-later",
                    self.base.m_prod_name,
                    self.base.m_product_version,
                    self.m_build_num,
                    self.m_build_date
                );
                println!("{}", version_string);
                self.base.do_exit(0);
            }
            "-license" | "-copyright" => {
                const LICENSE_NOTICE: &str = r#"PvZ-Portable

Copyright (C) 2026 Zhou Qiankang <wszqkzqk@qq.com>

SPDX-License-Identifier: LGPL-3.0-or-later
Some source files also include:
SPDX-License-Identifier: LGPL-3.0-or-later AND LicenseRef-PopCap

PvZ-Portable is distributed under the GNU Lesser General Public
License v3.0 or later. It is distributed in the hope that it will
be useful, but WITHOUT ANY WARRANTY; without even the implied
warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
"#;
                print!("{}", LICENSE_NOTICE);
                self.base.do_exit(0);
            }
            _ => {
                self.base.handle_cmd_line_param(param_name, _param_value);
            }
        }
    }

    /// PreDisplay 钩子 (对应 C++ SexyApp::PreDisplayHook)  
    pub fn pre_display_hook(&mut self) {
        log::trace!("SexyApp::pre_display_hook: PreDisplay 钩子");
        // 空实现，子类可覆写
    }

    /// PreTerminate 钩子 (对应 C++ SexyApp::PreTerminate)
    pub fn pre_terminate(&mut self) {
        log::info!("SexyApp::pre_terminate: PreTerminate 钩子");
        // 空实现，子类可覆写
    }

    /// 更新帧 (对应 C++ SexyApp::UpdateFrames)
    pub fn update_frames(&mut self) {
        log::trace!("SexyApp::update_frames: 更新帧");
        self.base.update_frames();
    }

    /// 获取 SEH 信息 (对应 C++ SexyApp::GetGameSEHInfo)
    pub fn get_game_seh_info(&self) -> String {
        log::trace!("SexyApp::get_game_seh_info: 获取 SEH 信息");
        format!(
            "{}Build Num: {}\r\nBuild Date: {}\r\n",
            self.base.get_game_seh_info(),
            self.m_build_num,
            self.m_build_date
        )
    }
}
