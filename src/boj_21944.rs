use std::io::Write;
use std::str::FromStr;
use std::collections::btree_set::BTreeSet;
use std::collections::btree_map::BTreeMap;

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub(crate) fn boj_21944() {
    let stdout = std::io::stdout();
    let out = &mut std::io::BufWriter::new(stdout.lock());

    let n = read_vec::<usize>()[0];

    // pair: {difficulty, problem_id}
    // index of problems means algorithm class
    let mut problems = vec![BTreeSet::<(i32, i32)>::new(); 110];

    // problem_id : {difficulty, algorithm_class}
    let mut problems_by_id = BTreeMap::<i32, (i32, i32)>::new();
    for _ in 0..n {
        // p, l, g
        let v = read_vec::<i32>();
        problems[v[2] as usize].insert((v[1], v[0]));
        problems_by_id.insert(v[0], (v[1], v[2]));
    }



    let q = read_vec::<usize>()[0];
    for _ in 0..q {
        let commands = read_vec::<String>();
        let command = &commands[0];
        match command.as_str() {
            "recommend" => {
                let g = commands[1].parse::<i32>().unwrap();
                let x = commands[2].parse::<i32>().unwrap();
                // find hardest, highest id problem in class g
                if x == 1 {
                    let (l, p) = problems[g as usize].last().unwrap();
                    writeln!(out, "{}", p).unwrap();
                }
                // find easiest, lowest id problem in class g
                else {
                    let (l, p) = problems[g as usize].first().unwrap();
                    writeln!(out, "{}", p).unwrap();
                }
            },
            "recommend2" => {
                let x = commands[1].parse::<i32>().unwrap();
                // find hardest, highest id problem in all classes
                if x == 1 {
                    let mut cur_p = 0;
                    let mut cur_l = 0;
                    for i in 1..110 {
                        if problems[i].len() > 0 {
                            let (l, p) = problems[i].last().unwrap();
                            if (l, p) > (&cur_l, &cur_p) {
                                cur_p = *p;
                                cur_l = *l;
                            }
                        }
                    }
                    writeln!(out, "{}", cur_p).unwrap();
                }
                // find easiest, lowest id problem in all classes
                else {
                    let mut cur_p = i32::MAX;
                    let mut cur_l = i32::MAX;
                    for i in 1..110 {
                        if problems[i].len() > 0 {
                            let (l, p) = problems[i].first().unwrap();
                            if (l, p) < (&cur_l, &cur_p) {
                                cur_p = *p;
                                cur_l = *l;
                            }
                        }
                    }
                    writeln!(out, "{}", cur_p).unwrap();
                }
            },
            "recommend3" => {
                let x = commands[1].parse::<i32>().unwrap();
                let l = commands[2].parse::<i32>().unwrap();
                // find easiest, lowest id problem in all classes, with difficulty >= l
                if x == 1 {
                    let mut cur_p = i32::MAX;
                    let mut cur_l = i32::MAX;
                    for i in 1..110 {
                        if problems[i].len() > 0 {
                            let iter = problems[i].range((l, 0)..).next();
                            if iter.is_none() {
                                continue;
                            }
                            let (l, p) = iter.unwrap();
                            if (l, p) < (&cur_l, &cur_p) {
                                cur_p = *p;
                                cur_l = *l;
                            }
                        }
                    }
                    if cur_p == i32::MAX {
                        cur_p = -1;
                    }
                    writeln!(out, "{}", cur_p).unwrap();
                }
                else {
                    // find hardest, highest id problem in all classes, with difficulty < l
                    let mut cur_p = 0;
                    let mut cur_l = 0;
                    for i in 1..110 {
                        if problems[i].len() > 0 {
                            let iter = problems[i].range(..(l, 0)).next_back();
                            if iter.is_none() {
                                continue;
                            }
                            let (l, p) = iter.unwrap();
                            if (l, p) > (&cur_l, &cur_p) {
                                cur_p = *p;
                                cur_l = *l;
                            }
                        }
                    }
                    if cur_p == 0 {
                        cur_p = -1;
                    }
                    writeln!(out, "{}", cur_p).unwrap();
                }
            },
            "add" => {
                let p = commands[1].parse::<i32>().unwrap();
                let l = commands[2].parse::<i32>().unwrap();
                let g = commands[3].parse::<i32>().unwrap();
                problems[g as usize].insert((l, p));
                problems_by_id.insert(p, (l, g));
            },
            "solved" => {
                let p = commands[1].parse::<i32>().unwrap();
                let (l, g) = problems_by_id.remove(&p).unwrap();
                problems[g as usize].remove(&(l, p));
            }
            _ => {
                // Handle all other commands here
            }
        }
    }
}