// 对应 C++ 中的 SexyVector.h - 2D/3D 向量
// 对应 C++ 中的 SexyMatrix.h / SexyMatrix.cpp - 3x3 变换矩阵

// 二维向量
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0 } }

    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vec2 {
        let len = self.length();
        if len > 0.0 {
            Vec2 { x: self.x / len, y: self.y / len }
        } else {
            Vec2::zero()
        }
    }
}

// 三维向量
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0, z: 0.0 } }
}

// 3x3 变换矩阵
#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
    pub m: [[f32; 3]; 3],
}

impl Matrix3 {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translate(dx: f32, dy: f32) -> Self {
        Self {
            m: [
                [1.0, 0.0, dx],
                [0.0, 1.0, dy],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            m: [
                [sx, 0.0, 0.0],
                [0.0, sy, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            m: [
                [cos, -sin, 0.0],
                [sin, cos, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn transform(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.m[0][0] * x + self.m[0][1] * y + self.m[0][2],
            self.m[1][0] * x + self.m[1][1] * y + self.m[1][2],
        )
    }
}

impl std::ops::Mul for Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: Matrix3) -> Matrix3 {
        let mut result = Matrix3 { m: [[0.0; 3]; 3] };
        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j];
            }
        }
        result
    }
}
