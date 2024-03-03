use std::{cmp, collections::VecDeque, io, ptr::read, str::FromStr};
use std::io::Write;

fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

fn get_angle(x: i64, y: i64) -> f64{
    let mut ret = (y as f64).atan2(x as f64);
    if ret < 0. {
        ret += 2. * std::f64::consts::PI;
    }
    return ret;
}

pub(crate) fn boj_16115() {
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock()); // 3

    let n = read_vec::<i64>()[0];
    let mut dots = vec![vec![0 as i64; 2]; n as usize];
    for i in 0 .. n {
        let xy = read_vec::<i64>();
        dots[i as usize][0] = xy[0];
        dots[i as usize][1] = xy[1];
    }

    let mut squared_dists = vec![0; n as usize];
    for i in 0 .. n as usize {
        squared_dists[i] = (dots[i][0] * dots[i][0] + dots[i][1] * dots[i][1]);;
    }

    let max_dist = squared_dists.iter().max().unwrap();
    let mut farthest_dots = Vec::<usize>::new();
    for i in 0 .. n as usize {
        if squared_dists[i] == *max_dist {
            farthest_dots.push(i as usize);
        }
    }

    if (farthest_dots.len() == 1) {
        writeln!(out, "360").unwrap();
        return;
    }

    // sort fartherst_dots by angle
    farthest_dots.sort_by(|a, b| {
        let angle_a = get_angle(dots[*a][0], dots[*a][1]);
        let angle_b = get_angle(dots[*b][0], dots[*b][1]);
        return angle_a.partial_cmp(&angle_b).unwrap();
    });

    let mut max_angle = 0.;
    for i in 0 .. farthest_dots.len() {
        let angle = get_angle(dots[farthest_dots[i]][0], dots[farthest_dots[i]][1]);
        let next_angle = get_angle(dots[farthest_dots[(i + 1) % farthest_dots.len()]][0], dots[farthest_dots[(i + 1) % farthest_dots.len()]][1]);
        let diff = next_angle - angle;
        if diff < 0. {
            max_angle = f64::max(max_angle, diff + 2. * std::f64::consts::PI);
        } else {
            max_angle = f64::max(max_angle, diff);
        }
    }

    // print with precision of 10^-8
    writeln!(out, "{:.8}", max_angle.to_degrees()).unwrap();
}