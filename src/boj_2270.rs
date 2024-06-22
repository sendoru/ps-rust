use std::{
    cmp::max,
    io::{self, Write},
    mem::swap,
    str::FromStr,
};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn modpow(mut a: i64, mut b: i64) -> i64 {
    let mut ret = 1;
    // let mut a = a;
    while b > 0 {
        if b % 2 == 1 {
            ret = (ret * a) % 1_000_000;
        }
        a = (a * a) % 1_000_000;
        b /= 2;
    }
    return ret;
}

fn solve(pos: &Vec<usize>, cur_disk: i64, move_to: i64, cur_ans: i64) -> i64 {
    if cur_disk == 0 {
        return cur_ans;
    }

    let cur_pos = pos[cur_disk as usize];
    let mut sub = 0;
    for i in 0..3 {
        if cur_pos != i && move_to != i as i64 {
            sub = i;
        }
    }

    if cur_pos == move_to as usize {
        return solve(pos, cur_disk - 1, move_to, cur_ans);
    }
    return solve(
        pos,
        cur_disk - 1,
        sub as i64,
        (cur_ans + modpow(2, cur_disk - 1)) % 1_000_000,
    );
}

pub(crate) fn boj_2270() {
    let mut n = read_vec::<i64>()[0];
    let mut cnts = read_vec::<i64>();
    let mut nos: Vec<Vec<i64>> = vec![vec![]; 3 as usize];
    let mut pos = vec![0; (n + 1) as usize];
    for i in 0..3 {
        nos[i] = read_vec::<i64>();
        for j in 0..nos[i].len() {
            pos[nos[i][j] as usize] = i;
        }
    }

    let mut base = 0;
    for i in 0..3 {
        if nos[i][0] == n {
            base = i;
            break;
        }
    }
    let ans = solve(&pos, n, base as i64, 0);
    writeln!(io::stdout(), "{}", base + 1).unwrap();
    writeln!(io::stdout(), "{}", ans).unwrap();

    let mut ans = 0;
}
