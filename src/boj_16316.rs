use std::io;
use std::io::Write;

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub(crate) fn boj_16316() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let n = read_vec::<usize>()[0];
    let mut v = read_vec::<String>();
    let mut ans = true;

    for i in 0..n {
        if v[i] != "mumble".to_string() && v[i].parse::<usize>().unwrap() != i + 1 {
            ans = false;
            break;
        }
    }

    if ans {
        writeln!(out, "makes sense").unwrap();
    } else {
        writeln!(out, "something is fishy").unwrap();
    }
}
