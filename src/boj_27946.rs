use std::{cmp, io, ptr::read, str::FromStr};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

#[derive(Clone)]
struct Trie {
    char_cnt: usize,
    children: Vec<Option<Box<Trie>>>,
    is_end: bool,
    trans_to_int: fn(char) -> usize,
}

impl Trie {
    fn new(char_cnt: usize, trans_to_int: fn(char) -> usize) -> Self {
        Trie {
            char_cnt,
            children: vec![None; char_cnt],
            is_end: false,
            trans_to_int,
        }
    }

    fn insert(&mut self, s: &str) {
        let mut cur = self;
        for c in s.chars() {
            let idx = (cur.trans_to_int)(c);
            if cur.children[idx].is_none() {
                cur.children[idx] = Some(Box::new(Trie::new(cur.char_cnt, cur.trans_to_int)));
            }
            cur = cur.children[idx].as_mut().unwrap();
        }
        cur.is_end = true;
    }

    fn search(&self, s: &str) -> bool {
        let mut cur = self;
        for c in s.chars() {
            let idx = (cur.trans_to_int)(c);
            if cur.children[idx].is_none() {
                return false;
            }
            cur = cur.children[idx].as_ref().unwrap();
        }
        cur.is_end
    }

    fn remove(&mut self, s: &str) -> bool {
        let mut cur = self;
        for c in s.chars() {
            let idx = (cur.trans_to_int)(c);
            if cur.children[idx].is_none() {
                return false;
            }
            cur = cur.children[idx].as_mut().unwrap();
        }
        if cur.is_end {
            cur.is_end = false;
            true
        } else {
            false
        }
    }
}

fn trans_to_int(c: char) -> usize {
    if b'0' <= c as u8 && c as u8 <= b'9' {
        (c as u8 - b'0') as usize
    } else if b'A' <= c as u8 && c as u8 <= b'Z' {
        (c as u8 - b'A' + 10) as usize
    } else {
        (c as u8 - b'a' + 36) as usize
    }
}

pub(crate) fn boj_27946() {}
