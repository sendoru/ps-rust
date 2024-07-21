use std::cmp;
use std::collections::VecDeque;
use std::io;

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn bfs_phase_1(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let dr = vec![-1, 1, 0, 0];
    let dc = vec![0, 0, -1, 1];
    let mut ret = board.clone();

    let mut q = VecDeque::new();
    let n = board.len();
    let m = board[0].len();

    let mut visited = vec![vec![false; m]; n];
    for r in 0..n {
        for c in 0..m {
            if board[r][c] == '.' {
                let mut adj_o_cnt = 0;
                for dir in 0..4 {
                    let nr = r as i32 + dr[dir];
                    let nc = c as i32 + dc[dir];
                    if nr < 0 || nr >= n as i32 || nc < 0 || nc >= m as i32 {
                        continue;
                    }
                    if board[nr as usize][nc as usize] == 'O' {
                        adj_o_cnt += 1;
                    }
                }
                if adj_o_cnt >= 2 {
                    q.push_back((r, c));
                    visited[r][c] = true;
                }
            }
        }
    }

    while !q.is_empty() {
        let (r, c) = q.pop_front().unwrap();
        ret[r][c] = 'O';
        for dir in 0..4 {
            let nr = r as i32 + dr[dir];
            let nc = c as i32 + dc[dir];
            if nr < 0 || nr >= n as i32 || nc < 0 || nc >= m as i32 {
                continue;
            }
            if !visited[nr as usize][nc as usize] && ret[nr as usize][nc as usize] == '.' {
                let mut adj_o_cnt = 0;
                for dir in 0..4 {
                    let nnr = nr as i32 + dr[dir];
                    let nnc = nc as i32 + dc[dir];
                    if nnr < 0 || nnr >= n as i32 || nnc < 0 || nnc >= m as i32 {
                        continue;
                    }
                    if ret[nnr as usize][nnc as usize] == 'O' {
                        adj_o_cnt += 1;
                    }
                }
                if adj_o_cnt >= 2 {
                    q.push_back((nr as usize, nc as usize));
                    visited[nr as usize][nc as usize] = true;
                }
            }
        }
    }

    ret
}

fn bfs_phase_2(board: &Vec<Vec<char>>, k: usize) -> usize {
    let mut ret = 0;
    let n = board.len();
    let m = board[0].len();
    let mut visited = vec![vec![false; m]; n];

    for r_start in 0..board.len() {
        for c_start in 0..board[0].len() {
            if board[r_start][c_start] == 'O' && !visited[r_start][c_start] {
                let mut q = VecDeque::new();
                let mut cc_nodes = Vec::new();
                q.push_back((r_start, c_start));
                visited[r_start][c_start] = true;
                while !q.is_empty() {
                    let (r, c) = q.pop_front().unwrap();
                    cc_nodes.push((r, c));
                    for dir in 0..4 {
                        let nr = r as i32 + [-1, 1, 0, 0][dir];
                        let nc = c as i32 + [0, 0, -1, 1][dir];
                        if nr < 0 || nr >= n as i32 || nc < 0 || nc >= m as i32 {
                            continue;
                        }
                        if !visited[nr as usize][nc as usize]
                            && board[nr as usize][nc as usize] == 'O'
                        {
                            q.push_back((nr as usize, nc as usize));
                            visited[nr as usize][nc as usize] = true;
                        }
                    }
                }

                cc_nodes.sort();
                let r_size = cc_nodes[cc_nodes.len() - 1].0 - cc_nodes[0].0 + 1;
                cc_nodes.sort_by_key(|(r, c)| c.clone());
                let c_size = cc_nodes[cc_nodes.len() - 1].1 - cc_nodes[0].1 + 1;
                if r_size > k && c_size > k {
                    ret += r_size * c_size;
                }
            }
        }
    }

    ret
}

pub(crate) fn boj_31782() {
    let nmk = read_vec::<usize>();
    let n = nmk[0];
    let m = nmk[1];
    let k = nmk[2];

    let mut board = vec![vec![]; n];
    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        // to char vec
        let s = s.trim().chars().collect::<Vec<_>>();
        board[i] = s;
    }

    let phase_1 = bfs_phase_1(&board);
    let ans = bfs_phase_2(&phase_1, k);
    println!("{}", ans);
}
