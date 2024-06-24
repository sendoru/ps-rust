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

pub(crate) fn boj_1326() {
    let n = read_vec::<i64>()[0];
    let a = read_vec::<i64>();
    let b = read_vec::<i64>();

    let mut dp = vec![std::i64::MAX; n as usize];
    dp[0] = 0;
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
