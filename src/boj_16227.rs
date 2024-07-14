use std::cmp::{self, Reverse};
use std::collections::BinaryHeap;
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

pub(crate) fn boj_16227() {
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock()); // 3

    let nk = read_vec::<usize>();
    let n = nk[0];
    let k = nk[1];

    let mut adj_list = vec![vec![]; n + 2];
    for _ in 0..k {
        let uvd = read_vec::<usize>();
        let u = uvd[0];
        let v = uvd[1];
        let d = uvd[2];
        adj_list[u].push((v, d));
        adj_list[v].push((u, d));
    }

    // dist[i][j] := i번째 정비소까지 마지막 청소 후 지난 시간이 j일 때, i번째 정비소까지 오는 데 걸리는 최소 시간
    let mut dist = vec![vec![i32::MAX; 101]; n + 2];

    // pq: (dist, node, from_last_cleaned_min)
    let mut pq = BinaryHeap::<cmp::Reverse<(i32, usize, i32)>>::new();
    dist[0][0] = 0;
    pq.push(Reverse((0, 0, 0)));
    while let Some(cmp::Reverse((d, u, from_last_cleaned_min))) = pq.pop() {
        if dist[u][from_last_cleaned_min as usize] < d {
            continue;
        }
        for &(v, w) in &adj_list[u] {
            // move to other node
            if from_last_cleaned_min + w as i32 <= 100 {
                let new_d = d + w as i32;
                if new_d < dist[v][(from_last_cleaned_min + w as i32) as usize] {
                    dist[v][(from_last_cleaned_min + w as i32) as usize] = new_d;
                    pq.push(Reverse((new_d, v, from_last_cleaned_min + w as i32)));
                }
            }
        }
        // clean car
        let new_d = d + 5;
        if new_d <= dist[u][0] {
            dist[u][0] = new_d;
            pq.push(Reverse((new_d, u, 0)));
        }
    }

    writeln!(out, "{}", dist[n + 1].iter().min().unwrap());
}
