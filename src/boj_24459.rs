use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::io;
use std::mem::swap;
use std::thread::panicking;

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn get_directed_adj_list(adj_list: &Vec<Vec<(usize, i64)>>, root: usize) -> Vec<Vec<(usize, i64)>> {
    let n = adj_list.len();
    let mut directed_adj_list = vec![vec![]; n];
    let mut visited = vec![false; n];
    fn get_directed_adj_list_internal(
        adj_list: &Vec<Vec<(usize, i64)>>,
        directed_adj_list: &mut Vec<Vec<(usize, i64)>>,
        visited: &mut Vec<bool>,
        node: usize,
    ) {
        visited[node] = true;
        for &(child, w) in adj_list[node].iter() {
            if visited[child] {
                continue;
            }
            directed_adj_list[node].push((child, w));
            get_directed_adj_list_internal(adj_list, directed_adj_list, visited, child);
        }
    }
    get_directed_adj_list_internal(adj_list, &mut directed_adj_list, &mut visited, root);
    directed_adj_list
}

fn solve(adj_list: &Vec<Vec<(usize, i64)>>, root: usize) -> (i64, i64) {
    let n = adj_list.len();
    let mut dp_max = vec![-1; n];
    let mut dp_min = vec![-1; n];

    let mut ans_max = vec![-1; n];
    let mut ans_min = vec![i64::MAX; n];

    // first, get the maximum distance of route whose start is leaf and end is current node
    fn solve_internal_max(
        adj_list: &Vec<Vec<(usize, i64)>>,
        dp: &mut Vec<i64>,
        node: usize,
    ) -> i64 {
        if adj_list[node].len() == 0 {
            dp[node] = 0;
            return 0;
        }
        let mut max_dist = 0;
        for &(child, w) in adj_list[node].iter() {
            max_dist = cmp::max(max_dist, solve_internal_max(adj_list, dp, child) + w);
        }
        dp[node] = max_dist;
        max_dist
    }

    solve_internal_max(adj_list, &mut dp_max, root);
    // and then, get the maximum distance of route whose start and end are leaf and the node is in the middle
    for i in 0..n {
        if adj_list[i].len() >= 2 {
            let mut dists = adj_list[i]
                .iter()
                .map(|&(child, w)| dp_max[child] + w)
                .collect::<Vec<i64>>();
            dists.sort_by(|a, b| b.cmp(a));
            ans_max[i] = dists[0] + dists[1];
        }
    }

    // do similar things for minimum distance
    fn solve_internal_min(
        adj_list: &Vec<Vec<(usize, i64)>>,
        dp: &mut Vec<i64>,
        node: usize,
    ) -> i64 {
        if adj_list[node].len() == 0 {
            dp[node] = 0;
            return 0;
        }
        let mut min_dist = i64::MAX;
        for &(child, w) in adj_list[node].iter() {
            min_dist = cmp::min(min_dist, solve_internal_min(adj_list, dp, child) + w);
        }
        dp[node] = min_dist;
        min_dist
    }

    solve_internal_min(adj_list, &mut dp_min, root);

    for i in 0..n {
        if adj_list[i].len() >= 2 {
            let mut dists = adj_list[i]
                .iter()
                .map(|&(child, w)| dp_min[child] + w)
                .collect::<Vec<i64>>();
            dists.sort();
            ans_min[i] = dists[0] + dists[1];
        }
    }

    (
        ans_max.iter().fold(0, |acc, &x| cmp::max(acc, x)),
        ans_min.iter().fold(i64::MAX, |acc, &x| cmp::min(acc, x)),
    )
}

pub(crate) fn boj_24459() {
    let n = read_vec::<usize>()[0];
    let mut adj_list: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for i in 0..n - 1 {
        let pqw = read_vec::<usize>();
        let p = pqw[0];
        let q = pqw[1];
        let w = pqw[2] as i64;
        adj_list[p].push((q, w));
        adj_list[q].push((p, w));
    }

    if n == 2 {
        println!("{}", adj_list[0][0].1);
        println!("{}", adj_list[0][0].1);
        return;
    }

    let mut root = -1;
    for i in 0..n {
        if adj_list[i].len() > 1 {
            root = i as i64;
            break;
        }
    }

    let mut adj_list = get_directed_adj_list(&adj_list, root as usize);

    let (ans_max, ans_min) = solve(&adj_list, root as usize);
    println!("{}", ans_max);
    println!("{}", ans_min);
}
