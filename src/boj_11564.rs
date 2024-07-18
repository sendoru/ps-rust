use std::{io, mem::swap};

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub(crate) fn boj_11564() {
    let kab = read_vec::<i64>();
    let k = kab[0];
    let mut a = kab[1];
    let mut b = kab[2];

    if a < 0 && b < 0 {
        swap(&mut a, &mut b);
        a = -a;
        b = -b;
    }

    let mut ans = 0;

    // case 1 : a < 0, b == 0
    if a < 0 && b == 0 {
        ans = -a / k + 1;
    }
    // case 2 : a < 0, b > 0
    else if a < 0 && b > 0 {
        ans = b / k + (-a / k) + 1;
    }
    // case 3: a == 0, b == 0
    else if a == 0 && b == 0 {
        ans = 1;
    }
    // case 4 : a == 0, b > 0
    else if a == 0 && b > 0 {
        ans = b / k + 1;
    }
    // case 5 : a > 0, b > 0
    else if a > 0 && b > 0 {
        ans = b / k - (a - 1) / k;
    }

    println!("{}", ans);
}
