use std::cmp::Reverse;
use std::io::Write;
use std::{io, num};

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub(crate) fn boj_28372() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let tcodi = read_vec::<i32>();
    let mut cnts: Vec<i128> = vec![0; 555];
    cnts[0] = 1;
    let side_cnts = [4, 6, 8, 12, 20];

    for i in 0..5 {
        for _ in 0..tcodi[i] {
            let mut cnts_tmp: Vec<i128> = vec![0; 555];
            for j in 1..=side_cnts[i] {
                for k in 0..555 {
                    if j + k < 555 {
                        cnts_tmp[j + k] += cnts[k];
                    }
                }
            }
            cnts = cnts_tmp;
        }
    }

    let mut ans = vec![];
    for i in 1..555 {
        if cnts[i] > 0 {
            ans.push((cnts[i], i));
        }
    }

    ans.sort_by_key(|x| (Reverse(x.0), x.1));
    for (cnt, i) in ans {
        writeln!(out, "{}", i);
    }
}
