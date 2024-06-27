use std::{
    cmp,
    collections::VecDeque,
    fs::read,
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

// #[derive(cmp::PartialEq)]
struct Line {
    a: i64,
    b: i64,
    start: f64,
}

impl Line {
    fn new(a: i64, b: i64, start: f64) -> Self {
        Line { a, b, start }
    }
}

// cmp::PartialOrd, cmp::PartialEq, cmp::Ord
impl cmp::PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl cmp::Ord for Line {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.start.partial_cmp(&other.start).unwrap()
    }
}

impl cmp::PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl cmp::Eq for Line {}

pub(crate) fn boj_17526() {
    let n = read_vec::<i64>()[0];
    n -= 1;
    let d = read_vec::<i64>();
    for i in 1..n {
        d[i] += d[i - 1];
    }
    let mut p = vec![0 as i64; n as usize];
    let mut s = vec![0 as i64; n as usize];

    for i in 0..n {
        let pisi = read_vec::<i64>();
        p[i as usize] = pisi[0];
        s[i as usize] = pisi[1];
    }

    let mut dp = vec![std::i64::MAX; n as usize];
    dp[0] = p[0] + d[0] * s[0];
    // dp[i] = min(dp[j] + p[j] + (d[i] - d[j]) * s[j]) (0 <= j < i)
    //       = min(dp[j] + d[i]s[j] + (p[j] - d[j] s[j])) (0 <= j < i)
    let mut lines = Vec::<Line>::new();
    lines.push(Line::new(b[0], 0, 0.));
    for i in 1..n as usize {
        let tmp = Line::new(0, 0, a[i] as f64);
        // upper bound
        let res = lines.partition_point(|x| x.start <= tmp.start) - 1;
        dp[i] = lines[res].a * a[i] + lines[res].b;
        let mut new_line = Line::new(b[i], dp[i], 0.);
        while let Some(last) = lines.last() {
            let x = (new_line.b - last.b) as f64 / (last.a - new_line.a) as f64;
            if x > last.start {
                new_line.start = x;
                lines.push(new_line);
                break;
            }
            lines.pop();
        }
    }

    println!("{}", dp[n as usize - 1]);
}
