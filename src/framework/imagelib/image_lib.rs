// 对应 C++ 中的 ImageLib.h / ImageLib.cpp
// 图像加载库 - 支持 PNG, JPEG, GIF, TGA 格式
//
// C++ 依赖: libpng, libjpeg, PakInterface
// Rust 映射: image crate (替代 libpng/libjpeg), sdl2::rwops (处理 pak 文件)
//
// ASSUMPTION: C++ 使用 libpng/libjpeg C 库进行图像编解码。
// Rust 版本使用 image crate 提供等价功能，但某些 jpeg2000 和 gif 动画
// 相关功能仅作为 FIXME 保留。

use std::fs;
use std::path::Path;

/// Image - 图像数据类 (对应 C++ ImageLib::Image)
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub bits: Vec<u32>,
}

impl Image {
    pub fn new() -> Self {
        Self { width: 0, height: 0, bits: Vec::new() }
    }

    pub fn get_width(&self) -> i32 { self.width }

    pub fn get_height(&self) -> i32 { self.height }

    pub fn get_bits(&self) -> &[u32] { &self.bits }

    pub fn get_bits_mut(&mut self) -> &mut [u32] { &mut self.bits }

    // Create Image from raw buffer (takes ownership)
    pub fn from_raw(width: i32, height: i32, bits: Vec<u32>) -> Self {
        Self { width, height, bits }
    }
}

// 全局变量 - 对应 C++ ImageLib 命名空间中的全局配置
static G_ALPHA_COMPOSE_COLOR: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0x00FFFFFF);
static G_AUTO_LOAD_ALPHA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

pub fn set_alpha_compose_color(color: u32) {
    G_ALPHA_COMPOSE_COLOR.store(color, std::sync::atomic::Ordering::Relaxed);
}

pub fn get_alpha_compose_color() -> u32 {
    G_ALPHA_COMPOSE_COLOR.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_auto_load_alpha(enable: bool) {
    G_AUTO_LOAD_ALPHA.store(enable, std::sync::atomic::Ordering::Relaxed);
}

pub fn get_auto_load_alpha() -> bool {
    G_AUTO_LOAD_ALPHA.load(std::sync::atomic::Ordering::Relaxed)
}

/// 从文件加载图像 (对应 C++ ImageLib::GetImage)
/// 根据文件扩展名自动选择解码器
pub fn get_image(filename: &str, look_for_alpha_image: bool) -> Option<Image> {
    log::info!("image_lib::get_image: 加载图像文件 {}", filename);
    let path = Path::new(filename);

    // Try loading from filesystem
    if let Ok(data) = fs::read(filename) {
        log::debug!("image_lib::get_image: 文件读取成功，大小 {} 字节", data.len());
        return decode_image_data(&data, path.extension().and_then(|e| e.to_str()));
    }

    log::warn!("image_lib::get_image: 文件 {} 读取失败", filename);
    // ASSUMPTION: If direct filesystem read fails, attempt via resource folder
    // (In full C++ implementation, this uses PakInterface for PAK file access)
    None
}

/// 根据数据解码图像 (对应 C++ 中 GetPNGImage/GetJPEGImage/GetGIFImage/GetTGAImage)
fn decode_image_data(data: &[u8], ext: Option<&str>) -> Option<Image> {
    log::debug!("image_lib::decode_image_data: 解码图像数据，大小 {} 字节，扩展名 {:?}", data.len(), ext);
    // Use the `image` crate to decode common formats
    // FIXME: GIF animation support not implemented
    // FIXME: JPEG2000 alpha channel detection not implemented

    let img = match image::load_from_memory(data) {
        Ok(img) => img.to_rgba8(),
        Err(e) => {
            log::error!("image_lib::decode_image_data: 图像解码失败: {}", e);
            return None;
        }
    };

    let (width, height) = img.dimensions();
    log::debug!("image_lib::decode_image_data: 图像尺寸 {}x{}", width, height);
    let raw = img.into_raw();

    // Convert RGBA bytes to u32 pixels
    let mut bits = Vec::with_capacity((width * height) as usize);
    for chunk in raw.chunks_exact(4) {
        let pixel = (chunk[3] as u32) << 24  // A
            | (chunk[0] as u32) << 16        // R
            | (chunk[1] as u32) << 8         // G
            | (chunk[2] as u32);             // B
        bits.push(pixel);
    }

    log::info!("image_lib::decode_image_data: 图像解码成功，{} 个像素", bits.len());
    Some(Image::from_raw(width as i32, height as i32, bits))
}

/// 保存 JPEG 图像 (对应 C++ ImageLib::WriteJPEGImage)
pub fn write_jpeg_image(filename: &str, _image: &Image) -> bool {
    log::warn!("image_lib::write_jpeg_image: JPEG 编码功能未实现，文件 {}", filename);
    // FIXME: JPEG encoding not implemented
    // C++ version uses libjpeg; Rust version could use image crate
    false
}

/// 保存 PNG 图像 (对应 C++ ImageLib::WritePNGImage)
pub fn write_png_image(filename: &str, _image: &Image) -> bool {
    log::warn!("image_lib::write_png_image: PNG 编码功能未实现，文件 {}", filename);
    // FIXME: PNG encoding not implemented
    false
}

/// 保存 TGA 图像 (对应 C++ ImageLib::WriteTGAImage)
pub fn write_tga_image(filename: &str, _image: &Image) -> bool {
    log::warn!("image_lib::write_tga_image: TGA 编码功能未实现，文件 {}", filename);
    // FIXME: TGA encoding not implemented
    false
}
