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

fn solve() {
    let nm = read_vec::<usize>();
    let n = nm[0];
    let m = nm[1];

    let mut ab = vec![vec![0, 0]; m];
    for i in 0..m {
        ab[i] = read_vec::<usize>();
        ab[i][0] -= 1;
        ab[i][1] -= 1;
    }

    // 정렬이나 하자
    // 끝나는 번호 순으로 정렬
    ab.sort_by(|a, b| [a[1], a[0]].partial_cmp(&[b[1], b[0]]).unwrap());
    let mut ans = 0;
    let mut used = vec![false; n];
    for i in 0..n {
        for j in ab[i][0]..ab[i][1] + 1 {
            if !used[j] {
                used[j] = true;
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}

pub(crate) fn boj_9576() {
    let t = read_vec::<usize>()[0];
    for _ in 0..t {
        solve();
    }
}
