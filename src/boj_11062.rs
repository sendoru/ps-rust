use std::io::Write; // Import the Write trait
use std::{str::FromStr, *};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub(crate) fn boj_11062() {
    let t = read_vec::<usize>()[0];
    for _ in 0..t {
        let n = read_vec::<usize>()[0];
        let mut v = read_vec::<i32>();
        // dp[i][j]: i번째부터 j번째 카드까지 남았을 때, 현재 턴인 사람이 얻을 수 있는 최대 점수
        let mut dp = vec![vec![0; n]; n];
        let mut prefix_sum = vec![0; n];
        prefix_sum[0] = v[0];
        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + v[i];
        }

        // recursive lambda
        fn solve(l: usize, r: usize, dp: &mut Vec<Vec<i32>>, prefix_sum: &Vec<i32>) -> i32 {
            if l == r {
                return prefix_sum[r] - if l == 0 { 0 } else { prefix_sum[l - 1] };
            }
            if dp[l][r] != 0 {
                return dp[l][r];
            }
            let sum = prefix_sum[r] - if l == 0 { 0 } else { prefix_sum[l - 1] };
            dp[l][r] = sum
                - std::cmp::min(
                    solve(l + 1, r, dp, prefix_sum),
                    solve(l, r - 1, dp, prefix_sum),
                );
            dp[l][r]
        }
        writeln!(io::stdout(), "{}", solve(0, n - 1, &mut dp, &prefix_sum)).unwrap();
    }
}
