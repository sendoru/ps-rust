use std::io::Write;
use std::{cmp, collections::BinaryHeap, collections::VecDeque, io, ptr::read, str::FromStr};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub(crate) fn boj_5529() {
    let mnk = read_vec::<usize>();
    let mut dots = vec![vec![0, 0, 0]; mnk[2]];
    for i in 0..mnk[2] {
        let xy = read_vec::<i64>();
        dots[i][0] = xy[0];
        dots[i][1] = xy[1];
        dots[i][2] = i as i64;
    }

    let mut adj_list_hor = vec![vec![]; mnk[2]];
    let mut adj_list_ver = vec![vec![]; mnk[2]];
    dots.sort_by(|a, b| [a[1], a[0]].cmp(&[b[1], b[0]]));
    for i in 1..dots.len() {
        if dots[i - 1][1] == dots[i][1] {
            adj_list_ver[dots[i - 1][2] as usize].push(dots[i][2]);
            adj_list_ver[dots[i][2] as usize].push(dots[i - 1][2]);
        }
    }
    dots.sort_by(|a, b| [a[0], a[1]].cmp(&[b[0], b[1]]));
    for i in 1..dots.len() {
        if dots[i - 1][0] == dots[i][0] {
            adj_list_hor[dots[i - 1][2] as usize].push(dots[i][2]);
            adj_list_hor[dots[i][2] as usize].push(dots[i - 1][2]);
        }
    }

    dots.sort_by(|a, b| [a[2]].cmp(&[b[2]]));

    let mut dist_hor = vec![i64::MAX; mnk[2]];
    let mut dist_ver = vec![i64::MAX; mnk[2]];
    let mut pq = BinaryHeap::new();
    for i in 0..dots.len() {
        if dots[i][0] == 1 {
            dist_hor[dots[i][2] as usize] = dots[i][1] - 1;
            pq.push(cmp::Reverse((dots[i][1] - 1, dots[i][2], 0)));
        }
    }

    while !pq.is_empty() {
        let cmp::Reverse((dist, node, dir)) = pq.pop().unwrap();
        // currently horizontal doors are open
        if dir == 0 {
            // do not push switch, just move to horizontally connected nodes
            for &next in adj_list_hor[node as usize].iter() {
                let next_dist = dist + (dots[next as usize][1] - dots[node as usize][1]).abs();
                if dist_hor[next as usize] > next_dist {
                    dist_hor[next as usize] = next_dist;
                    pq.push(cmp::Reverse((next_dist, next, 0)));
                }
            }
            // push switch
            if dist_ver[node as usize] > dist + 1 {
                dist_ver[node as usize] = dist + 1;
                pq.push(cmp::Reverse((dist + 1, node, 1)));
            }
        }
        // currently vertical doors are open
        else {
            // do not push switch, just move to vertically connected nodes
            for &next in adj_list_ver[node as usize].iter() {
                let next_dist = dist + (dots[next as usize][0] - dots[node as usize][0]).abs();
                if dist_ver[next as usize] > next_dist {
                    dist_ver[next as usize] = next_dist;
                    pq.push(cmp::Reverse((next_dist, next, 1)));
                }
            }
            // push switch
            if dist_hor[node as usize] > dist + 1 {
                dist_hor[node as usize] = dist + 1;
                pq.push(cmp::Reverse((dist + 1, node, 0)));
            }
        }
    }

    let mut ans = i64::MAX;
    for i in 0..dots.len() {
        // x coordinate is same as destination and horizontal door should be open
        if dots[i][0] == mnk[0] as i64 && dist_hor[i] != i64::MAX {
            ans = cmp::min(ans, dist_hor[i] + mnk[1] as i64 - dots[i][1]);
        }
        // y coordinate is same as destination and vertical door should be open
        if dots[i][1] == mnk[1] as i64 && dist_ver[i] != i64::MAX {
            ans = cmp::min(ans, dist_ver[i] + mnk[0] as i64 - dots[i][0]);
        }
    }

    if ans == i64::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
