// 对应 C++ 中的 SharedImage.h / SharedImage.cpp
// 共享图像引用
//
// ASSUMPTION: C++ SharedImage 是对 Image 的引用计数包装。
// Rust 使用 Arc<MemoryImage> 实现等价功能。

use std::sync::Arc;
use crate::framework::graphics::memory_image::MemoryImage;

/// SharedImage - 共享图像 (对应 C++ Sexy::SharedImage)
pub struct SharedImage {
    pub image: Option<Arc<MemoryImage>>,
    pub ref_count: i32,
}

impl SharedImage {
    pub fn new() -> Self {
        Self { image: None, ref_count: 0 }
    }

    pub fn from_image(image: MemoryImage) -> Self {
        Self {
            image: Some(Arc::new(image)),
            ref_count: 1,
        }
    }

    pub fn add_ref(&mut self) -> i32 {
        self.ref_count += 1;
        self.ref_count
    }
}
