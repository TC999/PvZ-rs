// 对应 C++ 中的 Resources.h / Resources.cpp
// 资源 ID 枚举和资源加载

use crate::todlib::tod_string_file::TodStringFile;

// 资源 ID 枚举 - 对应 C++ 中的 Sexy::ResourceId
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ResourceId {
    IMAGE_BLANK_ID = 0,
    IMAGE_POPCAP_LOGO_ID = 1,
    IMAGE_PARTNER_LOGO_ID = 2,
    IMAGE_TITLESCREEN_ID = 3,
    IMAGE_LOADBAR_DIRT_ID = 4,
    IMAGE_LOADBAR_GRASS_ID = 5,
    IMAGE_PVZ_LOGO_ID = 6,
    IMAGE_REANIM_SODROLLCAP_ID = 7,
    FONT_BRIANNETOD16_ID = 8,
    SOUND_BUTTONCLICK_ID = 9,
    SOUND_LOADINGBAR_FLOWER_ID = 10,
    SOUND_LOADINGBAR_ZOMBIE_ID = 11,
    FONT_HOUSEOFTERROR28_ID = 12,
    FONT_HOUSEOFTERROR20_ID = 13,
    FONT_HOUSEOFTERROR16_ID = 14,
    FONT_TINYBOLD_ID = 15,
    FONT_CONTINUUMBOLD14_ID = 16,
    FONT_CONTINUUMBOLD14OUTLINE_ID = 17,
    FONT_DWARVENTODCRAFT12_ID = 18,
    FONT_DWARVENTODCRAFT15_ID = 19,
    FONT_DWARVENTODCRAFT18_ID = 20,
    // ... 更多资源 ID 在实际转换中需要补全
    NUM_RESOURCE_IDS = 21,
}

pub static mut g_need_recalc_variable_to_id_map: bool = false;

pub struct Resources {
    pub string_file: TodStringFile,
    pub loaded: bool,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            string_file: TodStringFile::new(),
            loaded: false,
        }
    }

    pub fn load_resources(&mut self) {
        // TODO: 加载资源文件
        self.string_file.load_file("resources/strings.txt");
        self.loaded = true;
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.string_file.get(key)
    }
}
