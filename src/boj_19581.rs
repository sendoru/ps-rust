use std::{cmp, collections::VecDeque, io, ptr::read, str::FromStr};

fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

fn get_dist(adj_list: &Vec<Vec<[i32; 2]>>, start: usize) -> Vec<i32> {
    let mut ret = vec![-1; adj_list.len()];
    let mut q = VecDeque::new();
    q.push_back(start);
    ret[start] = 0;

    while q.len() > 0 {
        let curr = q.pop_front().unwrap();
        for i in 0 .. adj_list[curr].len() {
            let next = adj_list[curr][i][0] as usize;
            let cost = adj_list[curr][i][1];
            if ret[next] == -1 {
                ret[next] = ret[curr] + cost;
                q.push_back(next);
            }
        }
    }

    return ret;
}

pub(crate) fn boj_19581() {
    let n = read_vec::<i32>()[0];
    let mut adj_list = vec![vec![]; n as usize];
    for _ in 0 .. n - 1 {
        let s_e_w = read_vec::<i32>();
        adj_list[s_e_w[0] as usize - 1].push([s_e_w[1] - 1, s_e_w[2]]);
        adj_list[s_e_w[1] as usize - 1].push([s_e_w[0] - 1, s_e_w[2]]);
    }

    let dist_from_0 = get_dist(&adj_list, 0);
    let farthest = dist_from_0.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
    let mut dist_from_farthest = get_dist(&adj_list, farthest);
    let another_farthest = dist_from_farthest.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
    let mut dist_from_another_farthest = get_dist(&adj_list, another_farthest);

    dist_from_farthest.sort();
    dist_from_another_farthest.sort();

    println!("{}", cmp::max(dist_from_farthest[dist_from_farthest.len() - 2], dist_from_another_farthest[dist_from_another_farthest.len() - 2]));
}