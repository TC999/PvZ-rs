// 对应 C++ 中的 MemoryImage.h / MemoryImage.cpp
// 内存中的像素图像缓冲区
//
// ASSUMPTION: C++ MemoryImage 是驻留在系统内存中的 Image 子类，
// 用于软件渲染和图像处理。Rust 版本使用 Image 类型和 Vec<u32> 替代。

/// MemoryImage - 内存图像 (对应 C++ Sexy::MemoryImage)
pub struct MemoryImage {
    pub width: i32,
    pub height: i32,
    pub bits: Vec<u32>,
    /// C++ 使用 std::set<MemoryImage*> 追踪所有实例。Rust 中简化为单例列表。
    pub shared: bool,
}

impl MemoryImage {
    pub fn new() -> Self {
        Self { width: 0, height: 0, bits: Vec::new(), shared: false }
    }

    pub fn create(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.bits.resize((width * height) as usize, 0);
    }

    pub fn get_width(&self) -> i32 { self.width }
    pub fn get_height(&self) -> i32 { self.height }
    pub fn get_bits(&self) -> &[u32] { &self.bits }
    pub fn get_bits_mut(&mut self) -> &mut [u32] { &mut self.bits }
}
