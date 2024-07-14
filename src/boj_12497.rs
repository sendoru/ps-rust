use std::io::Write;
use std::{io, net};

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub(crate) fn boj_12497() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let t = read_vec::<usize>()[0];
    for test_case in 0..t {
        let n = read_vec::<usize>()[0];
        let mut match_res = vec![vec![]; n];
        for i in 0..n {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            match_res[i] = buf.to_string().chars().collect();
        }

        let mut wp = vec![0.0; n];
        let mut owp = vec![0.0; n];
        let mut oowp = vec![0.0; n];

        // wp
        for i in 0..n {
            let mut cnt = 0;
            for j in 0..n {
                if match_res[i][j] != '.' {
                    cnt += 1;
                    if match_res[i][j] == '1' {
                        wp[i] += 1.;
                    }
                }
            }
            wp[i] /= cnt as f64;
        }

        // owp
        for me in 0..n {
            let mut cnt = 0;
            for op in 0..n {
                let mut op_wp = 0.;
                let mut op_match_cnt = 0;
                if me == op {
                    continue;
                }
                if match_res[me][op] == '.' {
                    continue;
                }
                cnt += 1;
                for opop in 0..n {
                    if op == opop || me == opop {
                        continue;
                    }
                    if match_res[op][opop] != '.' {
                        op_match_cnt += 1;
                        op_wp += if match_res[op][opop] == '1' { 1. } else { 0. };
                    }
                }
                op_wp /= op_match_cnt as f64;
                owp[me] += op_wp;
            }
            owp[me] /= cnt as f64;
        }

        // oowp
        for i in 0..n {
            let mut cnt = 0;
            for j in 0..n {
                if match_res[i][j] != '.' {
                    cnt += 1;
                    oowp[i] += owp[j];
                }
            }
            oowp[i] /= cnt as f64;
        }

        writeln!(out, "Case #{}:", test_case + 1).unwrap();
        for i in 0..n {
            writeln!(out, "{}", 0.25 * wp[i] + 0.5 * owp[i] + 0.25 * oowp[i]).unwrap();
        }
    }
}
