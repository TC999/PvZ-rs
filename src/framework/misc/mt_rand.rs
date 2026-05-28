// 对应 C++ 中的 SexyAppFramework/misc/MTRand.h 和 MTRand.cpp
// 梅森旋转算法随机数生成器

use rand::Rng;

pub struct MTRand {
    rng: rand::rngs::StdRng,
}

impl MTRand {
    pub fn new() -> Self {
        use rand::SeedableRng;
        Self {
            rng: rand::rngs::StdRng::from_entropy(),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        use rand::SeedableRng;
        Self {
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    // 生成 [0, range) 之间的随机整数
    pub fn next_int(&mut self, range: i32) -> i32 {
        if range <= 0 { return 0; }
        self.rng.gen_range(0..range)
    }

    // 生成 [0, 1) 之间的随机浮点数
    pub fn next_float(&mut self) -> f64 {
        self.rng.gen::<f64>()
    }

    // 生成 [min, max) 之间的随机浮点数
    pub fn next_float_range(&mut self, min: f64, max: f64) -> f64 {
        self.rng.gen_range(min..max)
    }
}
