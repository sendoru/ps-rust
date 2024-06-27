use std::io::Write; // Import the Write trait
use std::{cmp, str::FromStr, *};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn solve(v: &Vec<i32>) -> i64 {
    let n = v.len() - 1;
    // dp[i]: 마지막 정지 층이 i일 때 불만 합의 최솟값
    let mut dp = vec![i64::MAX; n + 1];
    let mut prefix_sum = vec![0 as i64; n + 1];
    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + v[i] as i64;
    }
    dp[0] = 0;

    for i in 1..=n {
        let mut tmp = 0;
        for j in (1..i).rev() {
            // i번째 층 통과 -> i번째 층 이하에서 내리고 싶었는데 못 내린 승객들 불만 1씩 증가
            dp[i] = cmp::min(dp[i], dp[j] + tmp);
            tmp += (v[j] as i64) * (i as i64 - j as i64);
        }
        dp[i] = cmp::min(dp[i], tmp);
        // i번째 층 정지 -> i번째 층 이상에서 내리고 싶은 승객들 불만 1씩 증가
        dp[i] += prefix_sum[n] - prefix_sum[i];
    }

    dp[n]
}

pub(crate) fn boj_10284() {
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock()); // 3

    let t = read_vec::<usize>()[0];
    for _ in 0..t {
        let n = read_vec::<usize>()[0];
        let mut v = [vec![0], read_vec::<i32>()].concat();
        while v.len() > 0 && v[v.len() - 1] == 0 {
            v.pop();
        }
        if v.len() == 0 {
            writeln!(out, "{}", 0).unwrap();
            continue;
        }
        writeln!(out, "{}", solve(&v)).unwrap();
    }
}
