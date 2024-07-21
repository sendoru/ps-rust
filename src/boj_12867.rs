use std::cmp;
use std::collections::HashSet;
use std::io;

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

pub(crate) fn boj_12867() {
    let n = read_vec::<usize>()[0];
    let m = read_vec::<usize>()[0];
    let mut axis = read_vec::<usize>();

    let mut signs = String::new();
    io::stdin().read_line(&mut signs).unwrap();
    let signs: Vec<char> = signs.trim().chars().collect();

    let mut axis_indexs = vec![];
    for i in 0..m {
        axis_indexs.push((axis[i], i));
    }
    axis.sort();
    axis_indexs.sort();

    axis_indexs[0].0 = 0;
    for i in 1..m {
        if axis[i] == axis[i - 1] {
            axis_indexs[i].0 = axis_indexs[i - 1].0;
        } else {
            axis_indexs[i].0 = axis_indexs[i - 1].0 + 1;
        }
    }

    axis_indexs.sort_by_key(|x| x.1);

    let mut coord_set = HashSet::new();
    let mut coord_cur = vec![0; m];
    coord_set.insert(coord_cur.clone());
    for i in 0..m {
        if signs[i] == '+' {
            coord_cur[axis_indexs[i].0] += 1;
        } else {
            coord_cur[axis_indexs[i].0] -= 1;
        }
        if coord_set.contains(&coord_cur) {
            println!("0");
            return;
        }
        coord_set.insert(coord_cur.clone());
    }
    println!("1");
}
