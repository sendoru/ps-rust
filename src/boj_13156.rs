use std::{cmp, io, mem::swap, vec};

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub(crate) fn boj_13156() {
    let cm = read_vec::<usize>();
    let c = cm[0];
    let m = cm[1];

    let mut dp = vec![vec![i32::MIN; c + 1]; m + 1];
    let mut p = vec![vec![0]; m + 1];
    for i in 1..m + 1 {
        let mut temp = read_vec::<i32>();
        p[i].append(&mut temp);
    }
    dp[0][0] = 0;
    for i in 1..m + 1 {
        // i - 1번째 구매자까지 총 j개의 상품 구매
        for j in 0..c + 1 {
            // i번째 구매자가 k개의 상품 구매
            for k in 0..c + 1 {
                if j + k <= c {
                    dp[i][j + k] = cmp::max(dp[i][j + k], dp[i - 1][j] + p[i][k]);
                }
            }
        }
    }

    println!("{}", dp[m].iter().max().unwrap());
}
