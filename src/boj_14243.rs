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

pub(crate) fn boj_14243() {
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock()); // 3

    let mut s = read_vec::<String>()[0].chars().collect::<Vec<_>>();

    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    for ch in &s {
        match ch {
            'A' => a += 1,
            'B' => b += 1,
            'C' => c += 1,
            _ => unreachable!(),
        }
    }

    let mut ans = String::new();
    let mut after_last_b = 100;
    let mut after_last_c = 100;
    for i in 0..s.len() {
        // C를 넣을 수 있고, (B를 넣을 수 없거나 B를 넣을 수 있지만 C를 넣는 것이 더 유리한 경우)
        // (B를 넣을 수 있지만 C를 넣는 것이 더 유리한 경우) == (지금 C를 넣어도 나중에 B를 넣는 데 문제가 없는 경우)
        // B를 b개 넣기 위해서는, 최소 2 * b - 1 길이의 문자열이 필요하다.
        // ex: k == 5 -> BABAB
        if c > 0 && after_last_c >= 2 && (after_last_b < 1 || (s.len() as i32) - (i as i32) - 1 >= (2 * b - 1)) {
            ans.push('C');
            c -= 1;
            after_last_c = 0;
            after_last_b += 1;
        } else if b > 0 && after_last_b >= 1 {
            ans.push('B');
            b -= 1;
            after_last_b = 0;
            after_last_c += 1;
        } else {
            ans.push('A');
            a -= 1;
            after_last_b += 1;
            after_last_c += 1;
        }
    }

    if a == 0 && b == 0 && c == 0 {
        writeln!(out, "{}", ans);
    } else {
        writeln!(out, "-1");
    }
}