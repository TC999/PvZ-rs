// 对应 C++ 中的 SexyAppFramework/misc/Rect.h - TRect<T>
// 矩形结构体

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TRect<T = i32> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: Copy> TRect<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self { x, y, width, height }
    }
}

impl TRect<i32> {
    pub fn zero() -> Self {
        Self { x: 0, y: 0, width: 0, height: 0 }
    }

    pub fn left(&self) -> i32 { self.x }
    pub fn top(&self) -> i32 { self.y }
    pub fn right(&self) -> i32 { self.x + self.width }
    pub fn bottom(&self) -> i32 { self.y + self.height }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.right() && py >= self.y && py < self.bottom()
    }

    pub fn inflate(&mut self, dx: i32, dy: i32) {
        self.x -= dx;
        self.width += dx * 2;
        self.y -= dy;
        self.height += dy * 2;
    }

    pub fn inset(&mut self, dx: i32, dy: i32) {
        self.inflate(-dx, -dy);
    }
}
