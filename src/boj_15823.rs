use std::{cmp, collections::VecDeque, io, ptr::read, str::FromStr};
use std::io::Write;

fn read_vec<T>() -> Vec<T> where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

pub(crate) fn boj_15823() {
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock()); // 3

    let nm = read_vec::<i32>();
    let cards = read_vec::<i32>();

    let mut l = 1;
    let mut r = nm[0] / nm[1];
    let mut ans = 1;

    while l <= r {
        let mid = (l + r) / 2;
        let mut cnts = vec![0; (500_001) as usize];
        let mut pack_cnt = 0;
        let mut l_ptr = 0;
        let mut r_ptr = 0;

        while r_ptr < nm[0] {
            cnts[cards[r_ptr as usize] as usize] += 1;
            // move l_ptr to right until cnts[cards[r_ptr]] == 1
            while cnts[cards[r_ptr as usize] as usize] == 2 {
                cnts[cards[l_ptr as usize] as usize] -= 1;
                l_ptr += 1;
            }
            
            if r_ptr - l_ptr + 1 == mid {
                pack_cnt += 1;
                // move both l_ptr and r_ptr to r_ptr + 1
                while l_ptr <= r_ptr {
                    cnts[cards[l_ptr as usize] as usize] -= 1;
                    l_ptr += 1;
                }
                r_ptr += 1;
            }
            else {
                r_ptr += 1;
            }
        }

        if pack_cnt >= nm[1] {
            ans = std::cmp::max(ans, mid);
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    writeln!(out, "{}", ans).unwrap();
}