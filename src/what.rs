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

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    let buff = buf.trim_end();
    let t: i32 = buf.trim_end().parse().unwrap();
    for _ in 0 .. t {
        let nm = read_vec::<i32>();
        // n: 팔굽혀펴기 한 갯수
        // m: 득점 가능한 점수 종류
        // v: 득점 가능한 점수
        let (n, m) = (nm[0], nm[1]);
        let s = read_vec::<i32>();
        // ans[i][j]: i득점에 j번 푸시업이 가능하면 1, 아니면 0
        let mut vec: Vec<Vec<bool>> = vec![vec![false; (n + 1) as usize]; (555) as usize];
        // p득점에 q번 팔굽혀펴기가 가능하면
        // p + k 득점에 q + p + k번이 가능함

        // 0득점 0회부터 시작하자
        vec[0][0] = true;
            for i in 0 .. 555 {
                for j in 0 .. n + 1 {
                    if vec[i as usize][j as usize] == true {
                        for si in s.clone().into_iter() {
                            if (j + i + si <= n) {
                                vec[(i + si) as usize][(j + i + si) as usize] = true;
                            }
                        }
                    }
                }
            }

        let mut ans: i32 = -1;
        for i in (0 .. 500).rev() {
            if vec[i][n as usize] {
                ans = i as i32;
                break;
            }
        }

        println!("{ans}");
    }
}