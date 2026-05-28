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
        Self {
            phrase: phrase.to_string(),
            typed: String::new(),
            match_idx: 0,
            activated: false,
        }
    }

    pub fn add_char(&mut self, ch: char) -> bool {
        if self.activated { return false; }
        if self.match_idx < self.phrase.len() {
            if self.phrase.as_bytes()[self.match_idx] as char == ch.to_ascii_lowercase()
                || self.phrase.as_bytes()[self.match_idx] as char == ch.to_ascii_uppercase()
            {
                self.typed.push(ch);
                self.match_idx += 1;
                if self.match_idx >= self.phrase.len() {
                    self.activated = true;
                    return true;
                }
            } else {
                self.typed.clear();
                self.match_idx = 0;
            }
        }
        false
    }

    pub fn reset(&mut self) {
        self.typed.clear();
        self.match_idx = 0;
        self.activated = false;
    }
}
