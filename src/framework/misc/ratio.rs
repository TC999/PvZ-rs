// 对应 C++ 中的 Ratio.h / Ratio.cpp
// 分数有理数类型 - 用于精确的宽高比和帧率计算

#[derive(Debug, Clone, Copy)]
pub struct Ratio {
    pub numerator: i32,
    pub denominator: i32,
}

impl Ratio {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        assert!(denominator != 0);
        let mut r = Self { numerator, denominator };
        r.reduce();
        r
    }

    fn reduce(&mut self) {
        let g = gcd(self.numerator.abs(), self.denominator.abs());
        self.numerator /= g;
        self.denominator /= g;
        if self.denominator < 0 {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }
    }

    pub fn to_float(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    pub fn from_float(value: f64, max_denom: i32) -> Self {
        let sign = if value < 0.0 { -1 } else { 1 };
        let mut value = value.abs();
        let mut best_n = 0;
        let mut best_d = 1;
        let mut best_err = f64::MAX;

        for d in 1..=max_denom {
            let n = (value * d as f64).round() as i32;
            let approx = n as f64 / d as f64;
            let err = (value - approx).abs();
            if err < best_err {
                best_err = err;
                best_n = n * sign;
                best_d = d;
            }
        }
        Self::new(best_n, best_d)
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
