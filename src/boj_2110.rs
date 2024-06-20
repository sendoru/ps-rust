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

pub(crate) fn boj_2110() {
    let nm = read_vec::<usize>();
    let n = nm[0];
    let c = nm[1];

    let mut house = vec![0; n];
    for i in 0..n {
        house[i] = read_vec::<usize>()[0];
    }

    house.sort();

    let mut left = 1;
    let mut right = house[n - 1] - house[0];
    let mut ans = 0;
    while left <= right {
        let mid = (left + right) / 2;
        let mut cnt = 1;
        let mut last = house[0];
        for i in 1..n {
            if house[i] - last >= mid {
                cnt += 1;
                last = house[i];
            }
        }
        if cnt >= c {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    println!("{}", ans);
}
