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

fn read<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

pub(crate) fn boj_13137() {
    let n = read::<i64>();
    let mut p = read_vec::<i64>();
    let mut dp = vec![i64::MAX; (p[p.len() - 1] * 3) as usize];
    dp[0] = 0;

    for i in 1..dp.len() {
        for j in 0..p.len() {
            if i as i64 - p[j] >= 0 {
                dp[i] = std::cmp::min(dp[i], dp[i - p[j] as usize] + 1);
            }
        }
    }

    let mut greedy = vec![0; (p[p.len() - 1] * 3) as usize];
    for i in 1..greedy.len() {
        let mut remain = i as i64;
        for j in (0..p.len()).rev() {
            greedy[i] += remain / p[j];
            remain %= p[j];
        }
    }

    if dp == greedy {
        println!("Yes");
    } else {
        println!("No");
    }
}
