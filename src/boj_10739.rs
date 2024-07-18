use std::{io, mem::swap};

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub(crate) fn boj_10739() {
    let nk = read_vec::<usize>();
    let mut n = nk[0];
    let mut k = nk[1];
    let mut keys = vec![0; n];
    for i in 0..n {
        keys[i] = read_vec::<usize>()[0];
        keys[i] -= 1;
    }
    let mut inv_keys = vec![0; n];
    for i in 0..n {
        inv_keys[keys[i]] = i;
    }

    let mut ans = 0;

    // first move to unlock first door
    ans += inv_keys[0];
    k -= 1;
    let full_cycle_cnt = k / n;
    let remain = k % n;
    let mut move_per_cycle = 0;

    for i in 0..n {
        let cur = inv_keys[i];
        let next = inv_keys[(i + 1) % n];
        if i < remain {
            ans += (next + n - cur) % n;
        }
        move_per_cycle += (next + n - cur) % n;
    }

    ans += full_cycle_cnt * move_per_cycle;
    println!("{}", ans);
}
