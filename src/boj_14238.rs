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

pub(crate) fn boj_14238() {
    let mut s = read_vec::<String>()[0].chars().collect::<Vec<_>>();

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for ch in &s {
        match ch {
            'A' => a += 1,
            'B' => b += 1,
            'C' => c += 1,
            _ => unreachable!(),
        }
    }

    let n = s.len();
    // dp[i][j][k][l][m] : A를 i개, B를 j개, C를 k개 사용하고 마지막으로 사용한 문자가 l이고 그 전 문자가 m일 때 가능한 문자열
    // l, m은 원래 A, B, C 중 하나여야 하는데 인덱스를 줄이기 위해 0, 1, 2로 표현
    // A는 등장 빈도(?)에 제한이 없으므로 문자열 길이가 2 이하라서 마지막 문자 또는 그 전 문자가 정의되지 않으면 A라고 가정
    let mut dp = vec![vec![vec![vec![vec![String::new(); 3]; 3]; c + 1]; b + 1]; a + 1];
    let mut reachable = vec![vec![vec![vec![vec![false; 3]; 3]; c + 1]; b + 1]; a + 1];
    reachable[0][0][0][0][0] = true;

    // dp 계산
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                for l in 0..3 {
                    for m in 0..3 {
                        if !reachable[i][j][k][l][m] {
                            continue;
                        }
                        if i < a {
                            let mut next = dp[i][j][k][l][m].clone();
                            next.push('A');
                            // 가능한 문자열 뒤에 A를 붙이면 항상 가능한 문자열
                            reachable[i + 1][j][k][0][l] = true;
                            dp[i + 1][j][k][0][l] = next;
                        }
                        if j < b && l != 1 {
                            let mut next = dp[i][j][k][l][m].clone();
                            next.push('B');
                            // B로 끝나지 않는 가능한 문자열 뒤에 B를 붙이면 항상 가능한 문자열
                            reachable[i][j + 1][k][1][l] = true;
                            dp[i][j + 1][k][1][l] = next;
                        }
                        if k < c && l != 2 && m != 2 {
                            let mut next = dp[i][j][k][l][m].clone();
                            next.push('C');
                            reachable[i][j][k + 1][2][l] = true;
                            dp[i][j][k + 1][2][l] = next;
                        }
                    }
                }
            }
        }
    }

    // 정답 출력
    for i in 0..3 {
        for j in 0..3 {
            if reachable[a][b][c][i][j] {
                println!("{}", dp[a][b][c][i][j]);
                return;
            }
        }
    }

    println!("-1");
}