use std::{cmp, io, mem::swap, vec};

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn solve(
    board: &Vec<Vec<char>>,
    dp: &mut Vec<Vec<i32>>,
    trace: &mut Vec<(usize, usize)>,
    _r: i32,
    _c: i32,
) -> i32 {
    if _r < 0 || _r >= dp.len() as i32 || _c < 0 || _c >= dp[0].len() as i32 {
        for &(i, j) in trace.iter() {
            dp[i][j] = 0;
        }
        trace.clear();
        return 1;
    }

    let r = _r as usize;
    let c = _c as usize;

    if dp[r][c] == 0 || dp[r][c] == 1 {
        trace.clear();
        return dp[r][c];
    }
    // -2: visited in current trace
    // -1: not visited

    // cycle detected
    // trace에 있는 모든 값들을 0으로
    if dp[r][c] == -2 {
        dp[r][c] = 0;
        for &(i, j) in trace.iter() {
            dp[i][j] = 0;
        }
        trace.clear();
        return 0;
    }

    dp[r][c] = -2;
    trace.push((r, c));
    match board[r][c] {
        'U' => {
            let ret = solve(board, dp, trace, r as i32 - 1, c as i32);
            dp[r][c] = ret;
            return ret;
        }
        'D' => {
            let ret = solve(board, dp, trace, r as i32 + 1, c as i32);
            dp[r][c] = ret;
            return ret;
        }
        'L' => {
            let ret = solve(board, dp, trace, r as i32, c as i32 - 1);
            dp[r][c] = ret;
            return ret;
        }
        'R' => {
            let ret = solve(board, dp, trace, r as i32, c as i32 + 1);
            dp[r][c] = ret;
            return ret;
        }
        _ => {
            for &(i, j) in trace.iter() {
                dp[i][j] = 0;
            }
            trace.clear();
            return 0;
        }
    }

    return 0;
}

pub(crate) fn boj_17090() {
    let nm = read_vec::<usize>();
    let n = nm[0];
    let m = nm[1];
    let mut board = vec![Vec::<char>::new(); n];
    for i in 0..n {
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).unwrap();
        board[i] = tmp.trim().chars().collect();
    }

    let mut dp = vec![vec![-1; m]; n];
    let mut trace = Vec::<(usize, usize)>::new();

    let mut ans = 0;
    for r in 0..n {
        for c in 0..m {
            if dp[r][c] == -1 {
                solve(&board, &mut dp, &mut trace, r as i32, c as i32);
            }
            ans += dp[r][c];
        }
    }

    println!("{}", ans);
}
