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

pub(crate) fn boj_18231() {
    let nm = read_vec::<usize>();
    let n = nm[0];
    let m = nm[1];

    let mut adj = vec![vec![]; n + 1];
    for _ in 0..m {
        let ab = read_vec::<usize>();
        adj[ab[0]].push(ab[1]);
        adj[ab[1]].push(ab[0]);
    }

    let mut is_destroyed = vec![false; n + 1];
    let mut is_bombed = vec![false; n + 1];
    let k = read_vec::<usize>()[0];
    let destroyed = read_vec::<usize>();
    for &d in destroyed.iter() {
        is_destroyed[d] = true;
    }

    let mut ans = vec![];
    for i in 0..k {
        let mut is_ok = true;
        for &j in adj[destroyed[i]].iter() {
            if !is_destroyed[j] {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            is_bombed[destroyed[i]] = true;
            ans.push(destroyed[i]);
        }
    }

    let mut recon = vec![false; n + 1];
    for &a in ans.iter() {
        recon[a] = true;
        for &b in adj[a].iter() {
            recon[b] = true;
        }
    }

    if recon != is_destroyed {
        println!("-1");
    } else {
        println!("{}", ans.len());
        for &a in ans.iter() {
            print!("{} ", a);
        }
    }
}