// 对应 C++ 中的 SexyAppFramework/Common.h 和 Common.cpp

pub type ulong = u64;
pub type uint = u32;
pub type ushort = u16;
pub type uchar = u8;

pub fn max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

pub fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}
