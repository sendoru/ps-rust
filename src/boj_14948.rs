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

fn solve_bfs(board: &Vec<Vec<i32>>, bound: i32) -> bool {
    // return if it's possible to move from (0, 0) to (n - 1, m - 1), only visitng cells with value <= bound
    // only onext_ce, it is possible to teleport for 2 cells straight
    let dr = [1, -1, 0, 0];
    let dc = [0, 0, 1, -1];
    let dr_teleport = [2, -2, 0, 0];
    let dc_teleport = [0, 0, 2, -2];
    let n = board.len();
    let m = board[0].len();
    // use 3 dim vector for visited to store if it's visited with teleport or not
    let mut visited = vec![vec![vec![false; 2]; m]; n];
    let mut q = VecDeque::new();
    q.push_back((0, 0, false));
    visited[0][0][0] = true;

    while q.len() > 0 {
        let (r, c, teleported) = q.pop_front().unwrap();
        if r == n - 1 && c == m - 1 {
            return true;
        }

        for i in 0 .. 4 {
            let next_r = r as i32 + dr[i];
            let next_c = c as i32 + dc[i];
            if next_r < 0 || next_r >= n as i32 || next_c < 0 || next_c >= m as i32 {
                continue;
            }
            if board[next_r as usize][next_c as usize] <= bound && !visited[next_r as usize][next_c as usize][teleported as usize] {
                visited[next_r as usize][next_c as usize][teleported as usize] = true;
                q.push_back((next_r as usize, next_c as usize, teleported));
            }
        }

        if !teleported {
            for i in 0 .. 4 {
                let next_r = r as i32 + dr_teleport[i];
                let next_c = c as i32 + dc_teleport[i];
                if next_r < 0 || next_r >= n as i32 || next_c < 0 || next_c >= m as i32 {
                    continue;
                }
                if board[next_r as usize][next_c as usize] <= bound && !visited[next_r as usize][next_c as usize][true as usize] {
                    visited[next_r as usize][next_c as usize][true as usize] = true;
                    q.push_back((next_r as usize, next_c as usize, true));
                }
            }
        }
    }
    return false;
}

pub(crate) fn boj_14948() {
    let nm = read_vec::<usize>();
    let mut board = vec![vec![0]; nm[0]];
    for i in 0 .. nm[0] {
        board[i] = read_vec::<i32>();
    }

    // binary search, board[0][0] is lower bound, and 1e9 is the upper bound
    let mut left = board[0][0];
    let mut right = 1_000_000_000;
    let mut ans = 1_000_000_000;
    while left <= right {
        let mid = (left + right) / 2;
        if solve_bfs(&board, mid) {
            ans = cmp::min(ans, mid);
            right = mid - 1;
        }
        else {
            left = mid + 1;
        }
    }
    println!("{}", ans);
}