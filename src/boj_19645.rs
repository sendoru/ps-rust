use std::{
    cmp::{self, max, min},
    io,
    ptr::read,
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

pub(crate) fn boj_19645() {
    let n = read_vec::<usize>()[0];
    let v = [vec![0], read_vec::<i32>()].concat();
    let mut prefix_sum = vec![0; n + 1];
    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + v[i];
    }
    let mut dp =
        vec![vec![vec![false; (prefix_sum[n] + 1) as usize]; (prefix_sum[n] + 1) as usize]; n + 1];

    dp[0][0][0] = true;
    // i: 햄버거의 인덱스
    for i in 1..dp.len() {
        // j: 제일 효용이 큰 사람의 효용이 j
        for j in 0..dp[i].len() {
            // k: 두 번째로 효용이 큰 사람의 효용이 k
            for k in 0..dp[i][j].len() {
                if !dp[i - 1][j][k] {
                    continue;
                }
                // i-1번째까지 봤을 때 제일 효용이 큰 사람에게 i번째 햄버거를 줌
                if (j + v[i] as usize) < dp[i].len() {
                    dp[i][j + v[i] as usize][k] = true;
                }
                // i-1번째까지 봤을 때 두 번째로 효용이 큰 사람에게 i번째 햄버거를 줌
                if (k + v[i] as usize) < dp[i].len() {
                    if (k + v[i] as usize) >= j {
                        dp[i][k + v[i] as usize][j] = true;
                    } else {
                        dp[i][j][k + v[i] as usize] = true;
                    }
                }
                // i-1번째까지 봤을 때 세 번째로 효용이 큰 사람에게 i번째 햄버거를 줌
                let temp = prefix_sum[i - 1] - j as i32 - k as i32;
                if temp < 0 {
                    continue;
                }
                if temp + v[i] < k as i32 {
                    dp[i][j][k] = true;
                } else if temp + v[i] < j as i32 && temp + v[i] >= k as i32 {
                    dp[i][j][temp as usize + v[i] as usize] = true;
                } else if temp + v[i] >= j as i32 {
                    dp[i][temp as usize + v[i] as usize][j] = true;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..dp[n].len() {
        for j in 0..dp[n][i].len() {
            if dp[n][i][j] {
                ans = max(ans, prefix_sum[n] - i as i32 - j as i32);
            }
        }
    }
    println!("{}", ans);
}
