use std::{
    cmp,
    collections::VecDeque,
    io,
    str::{self, FromStr},
};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn bfs(
    board: &Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    start: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    let mut q = VecDeque::new();
    q.push_back(start);
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        if visited[x][y] || board[x][y] != board[start.0][start.1] {
            continue;
        }
        visited[x][y] = true;
        ret.push((x, y));
        for (dx, dy) in [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ]
        .iter()
        {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= board.len() as i32 || ny < 0 || ny >= board[0].len() as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if board[nx][ny] == board[x][y] {
                q.push_back((nx, ny));
            }
        }
    }
    ret
}

// (true, true) -> both local maximum and minimum
// (true, false) -> local maximum
// (false, true) -> local minimum
// (false, false) -> neither local maximum nor minimum
fn check_local_extreme_for_dot(board: &Vec<Vec<i32>>, dot: (usize, usize)) -> (bool, bool) {
    let (x, y) = dot;
    let mut is_local_max = true;
    let mut is_local_min = true;
    // 8-direction
    for (dr, dc) in [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ]
    .iter()
    {
        let nx = x as i32 + dr;
        let ny = y as i32 + dc;
        if nx < 0 || nx >= board.len() as i32 || ny < 0 || ny >= board[0].len() as i32 {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if board[nx][ny] > board[x][y] {
            is_local_max = false;
        } else if board[nx][ny] < board[x][y] {
            is_local_min = false;
        }
    }

    (is_local_max, is_local_min)
}

fn check_local_extreme_for_group(
    board: &Vec<Vec<i32>>,
    group: &Vec<(usize, usize)>,
) -> (bool, bool) {
    let mut is_local_max = true;
    let mut is_local_min = true;
    for dot in group.iter() {
        let (is_dot_local_max, is_dot_local_min) = check_local_extreme_for_dot(board, *dot);
        is_local_max &= is_dot_local_max;
        is_local_min &= is_dot_local_min;
    }
    (is_local_max, is_local_min)
}

pub(crate) fn boj_8142() {
    let n = read_vec::<usize>()[0];
    let mut board = vec![vec![]; n];

    for i in 0..n {
        board[i] = read_vec::<i32>();
    }

    let mut visited = vec![vec![false; n]; n];
    let mut ans = (0, 0);
    for r in 0..n {
        for c in 0..n {
            if (visited[r][c]) {
                continue;
            }
            let mut group = bfs(&board, &mut visited, (r, c));
            let (is_local_max, is_local_min) = check_local_extreme_for_group(&board, &group);
            if is_local_max {
                ans.0 += 1;
            }
            if is_local_min {
                ans.1 += 1;
            }
        }
    }
    println!("{} {}", ans.0, ans.1);
}
