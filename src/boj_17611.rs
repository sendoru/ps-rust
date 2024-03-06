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

pub(crate) fn boj_17611() {
    let n = read_vec::<usize>()[0];
    let mut delta_x = vec![0; 1_000_100];
    let mut delta_y = vec![0; 1_000_100];

    let mut dots = vec![vec![0, 0]; n];
    for i in 0..n {
        dots[i] = read_vec::<i32>();
        dots[i][0] += 500_000;
        dots[i][1] += 500_000;
    }

    for i in 0..n {
        let curr = &dots[i];
        let next = &dots[(i + 1) % n];
        if curr[0] == next[0] {
            delta_y[cmp::min(next[1], curr[1]) as usize] += 1;
            delta_y[cmp::max(next[1], curr[1]) as usize] -= 1;
        } else {
            delta_x[cmp::min(next[0], curr[0]) as usize] += 1;
            delta_x[cmp::max(next[0], curr[0]) as usize] -= 1;
        }
    }

    let mut x_counts = vec![0; 1_000_100];
    let mut y_counts = vec![0; 1_000_100];
    x_counts[0] = delta_x[0];
    y_counts[0] = delta_y[0];
    for i in 1..1_000_100 {
        x_counts[i] = x_counts[i - 1] + delta_x[i];
        y_counts[i] = y_counts[i - 1] + delta_y[i];
    }

    let mut v_max = 0;
    let mut h_max = 0;
    for i in 0..1_000_100 {
        v_max = cmp::max(v_max, y_counts[i]);
        h_max = cmp::max(h_max, x_counts[i]);
    }

    println!("{}", cmp::max(v_max, h_max));
}
