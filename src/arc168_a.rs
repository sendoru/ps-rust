use std::{cmp, io, ptr::read, str::FromStr};

fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

pub(crate) fn arc168_a() {
    let nq = read_vec::<i32>();
    let n = nq[0];
    let q = nq[1];
    
    let mut logs = vec![vec![0, 0, 0]; n as usize];
    for i in 0 .. n {
        logs[i as usize] = read_vec::<i32>();
    }

    
}