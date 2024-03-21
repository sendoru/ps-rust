use std::{io, str::FromStr, cmp};
use std::collections::btree_map as map;
use std::collections::BinaryHeap as Heap;

fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

fn dijkstra (adj_list: &Vec<map::BTreeMap<usize, i32>>, g_edge_info: &map::BTreeMap<(usize, usize), (i32, i32)>, s: usize) -> Vec::<i32> {
    let mut dist = vec![i32::MAX; adj_list.len()];
    // dist[s] = 0;

    // 이거 max heap이라 거리 넣을때 -붙여서 넣어야함
    let mut pq: Heap<(i32, usize)> = Heap::new();

    pq.push((0, s));
    while !pq.is_empty() {
        let (mut d, here) = pq.pop().unwrap();
        d *= -1;
        if d >= dist[here] {
            continue;
        }
        dist[here] = d;
        for (there, w) in adj_list[here].iter() {
            let mut next_dist = d + w;
            if g_edge_info.contains_key(&(here, *there)) {
                let (g_time, g_w) = g_edge_info.get(&(here, *there)).unwrap();
                if g_time.clone() <= d && g_time.clone() + g_w.clone() > d {
                    next_dist = g_time.clone() + g_w.clone() + g_w.clone();
                }
            }
            if next_dist < dist[*there] {
                pq.push((-next_dist, *there));
            }
        }
    }
    return dist;
}

pub(crate) fn boj_2982() {
    let nm = read_vec::<usize>();
    let n = nm[0];
    let m = nm[1];
    let abkg = read_vec::<usize>();
    let a = abkg[0];
    let b = abkg[1];
    let k = abkg[2];
    let g = abkg[3];

    let mut gg = read_vec::<i32>();
    let mut adj_list: Vec<map::BTreeMap<usize, i32>> = vec![map::BTreeMap::new(); n + 1];
    for _ in 0..m {
        let uvw = read_vec::<usize>();
        let u = uvw[0];
        let v = uvw[1];
        let w = uvw[2];
        adj_list[u].insert(v, w as i32);
        adj_list[v].insert(u, w as i32);
    }

    let mut g_edge_info:map::BTreeMap<(usize, usize), (i32, i32)> = map::BTreeMap::new();
    let mut g_time = -(k as i32);
    for i in 0..g-1 {
        let u = gg[i] as usize;
        let v = gg[i + 1] as usize;
        let w = *adj_list[u].get(&v).unwrap();
        g_edge_info.insert((u, v), (g_time, w));
        g_edge_info.insert((v, u), (g_time, w));
        g_time += w;
    }

    // need to impl dijkstra
    let ans = dijkstra(&adj_list, &g_edge_info, a);
    println!("{}", ans[b]);
}