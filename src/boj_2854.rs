use std::{cmp, collections::VecDeque, io, ptr::read, str::FromStr};
use std::io::Write;

fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}



pub(crate) fn boj_2854() {
    let MOD7 = 1_000_000_007;
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock()); // 3
    let n = read_vec::<usize>()[0];
    let mut v1 = read_vec::<i64>();
    let mut v2 = read_vec::<i64>();
    v2.push(0);

    // dp[i][0] : 마지막에 v1[i]를 선택했을 때 경우의 수
    // dp[i][1] : 마지막에 v2[i]를 선택했을 때 경우의 수
    // dp[i][2] : 마지막에 v2[i - 1]을 선택했을 때 경우의 수
    let mut dp = vec![vec![0, 0, 0]; n];
    dp[0][0] = v1[0];
    dp[0][1] = v2[0];
    for i in 1 .. n as usize {
        dp[i][0] = (dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][2]) * v1[i] % MOD7;
        dp[i][1] = (dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][2]) * v2[i] % MOD7;
        dp[i][2] = ((dp[i - 1][0] + dp[i - 1][2]) * v2[i - 1] + dp[i - 1][1] * (v2[i - 1] - 1)) % MOD7;
    }

    writeln!(out, "{}", (dp[n - 1][0] + dp[n - 1][1] + dp[n - 1][2]) % MOD7).unwrap();
}