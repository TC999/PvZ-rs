// 对应 C++ 中的 Sexy.TodLib/TodList.h
// 双向链表 - 用于游戏对象的链接列表管理

use std::ptr::NonNull;

// C++ 中 TodList 是一个侵入式双向链表
// 在 Rust 中，我们使用标准库的 LinkedList 或 Vec 代替

pub type TodList<T> = Vec<T>;

pub trait TodListNode {
    fn next(&self) -> Option<&Self>;
    fn prev(&self) -> Option<&Self>;
}
