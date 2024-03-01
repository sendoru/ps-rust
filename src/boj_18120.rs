use std::{io, str::FromStr, cmp};

fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

pub(crate) fn boj_18120() {
    let na = read_vec::<i32>();
    let n = na[0] as usize;
    let a = na[1] as f64;

    let mut dw = vec![vec![0.; 2]; n];
    for i in 0 .. n {
        let xyw = read_vec::<i32>();
        dw[i][0] = ((xyw[0] * xyw[0] + xyw[1] * xyw[1]) as f64).powf(0.5);
        dw[i][1] = xyw[2] as f64;
    }

    dw.sort_by(|a, b| {
        a[0].total_cmp(&b[0])
    });

    dw.push(vec![f64::MAX, 0.]);

    // 반지름 r일 때, ci = r - di
    // 반지름을 dr만큼 늘릴 때, 원 안에 있던 농작물 가치의 합을 sw라고 하면 dr * sw만큼 전체 return이 늘어남
    // 
    let mut ans = 0.;
    let mut w_sum = dw[0][1];
    let mut ret = 0.;

    for i in 1 .. n + 1 {
        let opt_r = w_sum / (2. * a);
        if dw[i - 1][0] <= opt_r && opt_r <= dw[i][0] {
            let delta_r = opt_r - dw[i - 1][0];
            let opt_ret: f64 = ret + delta_r * w_sum;
            if opt_ret - a * opt_r * opt_r > ans {
                ans = opt_ret - a * opt_r * opt_r;
            }
        }

        let delta_r = dw[i][0] - dw[i - 1][0];
        ret += delta_r * w_sum;
        if ret - a * dw[i][0] * dw[i][0] > ans {
            ans = ret - a * dw[i][0] * dw[i][0];
        }
        w_sum += dw[i][1];
    }

    // 맨 바깥 것까지 포함됐을 때 edge case
    // 는 dw 맨 뒤에 무한히 먼 점 포함해서 처리함

    println!("{ans}");
}