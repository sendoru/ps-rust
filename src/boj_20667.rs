use std::io::Write;
use std::{cmp, collections::VecDeque, io, ptr::read, str::FromStr};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub(crate) fn boj_20667() {
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock()); // 3

    let nmk = read_vec::<usize>();
    // O(5 n^2 m) knapsack?
    // (N ≤ 100, M ≤ 1,000, K ≤ 100,000)
    // 다음 n 줄에는 다음과 같이 크롬 탭의 정보가 주어진다.
    // cpu, memory, priority (1 ≤ cpu ≤ M, 1 ≤ memory ≤ K, 1 ≤ priority ≤ 5)
    let mut tabs = vec![vec![0, 0, 0]; nmk[0]];
    for i in 0..nmk[0] {
        tabs[i] = read_vec();
    }
    // dp[i][j] : 우선순위 값을 i만큼 소비해서 j만큼의 cpu 사용량을 감소시켰을 때, 감소시킬 수 있는 메모리의 최대값
    // 근데 cpu 사용량을 m 이상 감소시킨 경우는 clipping? 비슷하게 하면 됨
    // -1: 도달 불가능 상태
    let mut dp = vec![vec![-1 as i64; nmk[1] + 1]; 5 * nmk[0] + 1];
    dp[0][0] = 0;
    for i in 0..nmk[0] {
        // 리버스 걸어야 중복계산이 안되는구나
        for priority in (0..5 * nmk[0] + 1).rev() {
            for cpu in (0..nmk[1] + 1).rev() {
                if dp[priority][cpu] == -1 {
                    continue;
                }
                // 현재 탭을 선택해서 상태 전이
                let next_priority = priority + tabs[i][2];
                let next_cpu = cmp::min(nmk[1], cpu + tabs[i][0]);
                let next_memory = dp[priority][cpu] + tabs[i][1] as i64;
                if next_priority <= 5 * nmk[0] {
                    dp[next_priority][next_cpu] =
                        cmp::max(dp[next_priority][next_cpu], next_memory);
                }
            }
        }
    }

    let mut ans = -1;
    for priority in 0..5 * nmk[0] + 1 {
        // 어차피 cpu 사용량 감소는 무조건 m 이상이어야 함
        // 마지막 행만 확인
        if dp[priority][nmk[1]] >= nmk[2] as i64 {
            ans = priority as i64;
            break;
        }
    }

    writeln!(out, "{}", ans);
}
