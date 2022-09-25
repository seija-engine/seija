use std::{str::Chars, collections::VecDeque};

#[derive(Debug,Clone,Copy)]
pub struct Position {
   pub line:u64,
   pub column:u64,
}

impl Default for Position {
    fn default() -> Self {
        Self { line: 1, column: 0 }
    }
}

pub struct LexString<'a> {
    source: &'a str,
    chars:Chars<'a>,
    pos:Position,
    cur_index:usize,
    byte_index:usize,
    ahead_count:usize,
    cache_list:VecDeque<char>,
    back_cache_count:usize
}

impl<'a> LexString<'a> {
    pub fn new(string: &'a str,back_cache_count:usize) -> LexString<'a> {
        LexString {
            source: string,
            chars:string.chars(),
            pos:Position::default(),
            cur_index:0,
            byte_index:0,
            ahead_count:0,
            cache_list:VecDeque::default(),
            back_cache_count,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let chr = if self.ahead_count > 0 {
            self.cur_index += 1;
            let sub_count = self.cache_list.len() - self.ahead_count;
            self.ahead_count -= 1;
            let ret_chr = self.cache_list[sub_count];
            self.byte_index += ret_chr.len_utf8();
            Some(ret_chr)
        } else {
            let next = self.chars.next();
            if let Some(chr) = next {
                if self.cache_list.len() >= self.back_cache_count {
                    self.cache_list.pop_front();
                }
                self.cache_list.push_back(chr);
                self.cur_index += 1;
                self.byte_index += chr.len_utf8();
            }
            next
        };
        if let Some(chr) = chr {
            if chr == '\n' {
                self.pos.line += 1;
                self.pos.column = 0;
            } else {
                self.pos.column += 1;
            }
        }
        chr
    }

    pub fn position(&self) -> Position { self.pos }

    pub fn lookahead(&mut self,count:usize) -> Option<char> {
        if self.ahead_count > count {
            let sub_len = self.cache_list.len() - self.ahead_count;
            return Some(self.cache_list[sub_len + count - 1]);
        } else {
            let add_count = count - self.ahead_count;
            for _ in 0..add_count {
              if let Some(chr) = self.chars.next() {
                 self.ahead_count += 1;
                 self.cache_list.push_back(chr);
              } else {
                 return None;
              }
            }
            let sub_len = self.cache_list.len() as i32 - self.ahead_count as i32;
            let idx = sub_len + count as i32 - 1;
            if idx >= self.cache_list.len() as i32 || idx < 0 {
                return None;
            }

            return Some(self.cache_list[idx as usize]);
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(chr) = self.lookahead(1) {
            if chr.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }
}
