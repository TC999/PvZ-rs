// 对应 C++ 中的 TypingCheck.h / TypingCheck.cpp
// 打字检测 - 用于作弊码输入检测

pub struct TypingCheck {
    pub phrase: String,
    pub typed: String,
    pub match_idx: usize,
    pub activated: bool,
}

impl TypingCheck {
    pub fn new(phrase: &str) -> Self {
        log::debug!("TypingCheck::new: 创建打字检测，短语 {}", phrase);
        Self {
            phrase: phrase.to_string(),
            typed: String::new(),
            match_idx: 0,
            activated: false,
        }
    }

    pub fn add_char(&mut self, ch: char) -> bool {
        log::trace!("TypingCheck::add_char: 添加字符 '{}'，当前匹配索引 {}", ch, self.match_idx);
        if self.activated {
            log::trace!("TypingCheck::add_char: 已激活，返回 false");
            return false;
        }
        if self.match_idx < self.phrase.len() {
            if self.phrase.as_bytes()[self.match_idx] as char == ch.to_ascii_lowercase()
                || self.phrase.as_bytes()[self.match_idx] as char == ch.to_ascii_uppercase()
            {
                self.typed.push(ch);
                self.match_idx += 1;
                log::trace!("TypingCheck::add_char: 字符匹配，新索引 {}", self.match_idx);
                if self.match_idx >= self.phrase.len() {
                    log::info!("TypingCheck::add_char: 作弊码激活！短语 {}", self.phrase);
                    self.activated = true;
                    return true;
                }
            } else {
                log::trace!("TypingCheck::add_char: 字符不匹配，重置");
                self.typed.clear();
                self.match_idx = 0;
            }
        }
        false
    }

    pub fn reset(&mut self) {
        log::debug!("TypingCheck::reset: 重置打字检测");
        self.typed.clear();
        self.match_idx = 0;
        self.activated = false;
    }
}
