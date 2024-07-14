use std::cmp;
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

fn is_parallel(a: (i32, i32), b: (i32, i32)) -> bool {
    return a.0 * b.1 == a.1 * b.0;
}

pub(crate) fn boj_2778() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let t = read_vec::<usize>()[0];
    for _ in 0..t {
        // line ax + by + c = 0
        let l1 = read_vec::<i32>();
        let l2 = read_vec::<i32>();
        let l3 = read_vec::<i32>();

        // normal vector
        let n1 = (l1[0], l1[1]);
        let n2 = (l2[0], l2[1]);
        let n3 = (l3[0], l3[1]);

        // check if there is parallel line
        if is_parallel(n1, n2) || is_parallel(n2, n3) || is_parallel(n3, n1) {
            writeln!(out, "0.0000");
            continue;
        }

        // get intersection point
        let p12_det = n1.0 * n2.1 - n1.1 * n2.0;
        let p12_x = (n1.1 * l2[2] - n2.1 * l1[2]) as f64 / p12_det as f64;
        let p12_y = (n2.0 * l1[2] - n1.0 * l2[2]) as f64 / p12_det as f64;
    }
}
