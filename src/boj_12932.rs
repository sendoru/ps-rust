use std::{io, str::FromStr, cmp};


fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

fn get_remain_dist (dp_table: &mut Vec::<Vec::<i32>>, i: usize, j: usize, v: &Vec<i32>) -> i32 {
    let n = v.len() - 1;
    let next = cmp::max(i, j) + 1;
    if next > n {
        return 0;
    }
    if dp_table[i][j] != i32::MAX / 4 {
        return dp_table[i][j];
    }
    let mut i_dist = 0;
    if i != 0 {
        i_dist = (v[next] - v[i]).abs();
    }
    let mut j_dist = 0;
    if j != 0 {
        j_dist = (v[next] - v[j]).abs();
    }
    let next_i = get_remain_dist(dp_table, next, j, v);
    let next_j = get_remain_dist(dp_table, i, next, v);
    if next_i + i_dist < next_j + j_dist {
        dp_table[i][j] = next_i + i_dist;
    } else {
        dp_table[i][j] = next_j + j_dist;
    }
    return dp_table[i][j];
}


pub(crate) fn boj_12932() {
    let n = read_vec::<usize>()[0];
    let mut v: Vec::<i32> = vec![0 as i32; 1];
    v.extend(read_vec::<i32>());

    // dp_table[i][j] := a가 마지막으로 선택한 인덱스가 i, b가 마지막으로 선택한 인덱스가 j일 때의 최소값
    let mut dp_table = vec![vec![i32::MAX / 4; n + 1]; n + 1];

    let ans = get_remain_dist(&mut dp_table, 0, 0, &v);

    println!("{}", ans);
}