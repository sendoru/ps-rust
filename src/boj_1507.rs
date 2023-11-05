use std::{io, str::FromStr, cmp};


fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

fn solve(adj_list: &Vec::<Vec::<i32>>) -> i32 {
    let INF = 998244353;

    let mut floyd_table = adj_list.to_vec();
    let mut res = 0;

    for _ in 0 .. floyd_table.len() {
        for m in 0 .. floyd_table.len() {
            for s in 0 .. floyd_table.len() {
                for e in 0 .. floyd_table.len() {
                    if (s != e && s != m && e != m && floyd_table[s][e] != INF && floyd_table[s][m] + floyd_table[m][e] == floyd_table[s][e]) {
                        floyd_table[s][e] = INF;
                        floyd_table[e][s] = INF;
                    }
                }
            }
        }
    }

    for s in 0 .. floyd_table.len() {
        for e in s + 1 .. floyd_table.len() {
            if (floyd_table[s][e] != INF) {
                res += floyd_table[s][e];
            }
        }
    }

    for m in 0 .. floyd_table.len() {
        for s in 0 .. floyd_table.len() {
            for e in 0 .. floyd_table.len() {
                floyd_table[s][e] = cmp::min(floyd_table[s][e], floyd_table[s][m] + floyd_table[m][e]);
            }
        }
    }

    if (floyd_table != adj_list.to_vec()) {
        return -1;
    }

    return res;
}

pub(crate) fn boj_1507() {


    let mut n = read_vec::<i32>()[0];
    let mut adj_list = vec![vec![0; n as usize]; n as usize];
    for i in 0 .. n {
        adj_list[i as usize] = read_vec::<i32>();
    }

    // 비트마스크로 간선의 존재 여부 표현
    let ans = solve(&adj_list);
    println!("{ans}");
}