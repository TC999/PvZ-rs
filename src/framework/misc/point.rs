// 对应 C++ 中的 SexyAppFramework/misc/Point.h - TPoint<T>
// 二维点结构体

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPoint<T = i32> {
    pub x: T,
    pub y: T,
}

impl<T> TPoint<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl TPoint<i32> {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}
