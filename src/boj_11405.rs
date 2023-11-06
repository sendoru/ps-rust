use std::{io, cmp, str::FromStr, collections::{self, VecDeque}};

fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}
struct GraphForMCMF {
    capacity: Vec<Vec<i64>>,
    cost: Vec<Vec<i64>>
}

impl GraphForMCMF {
    fn get_mcmf(&self, source: usize, sink: usize) -> i64 {
        let mut res: i64 = 0;
        let mut adj_list: Vec<Vec<usize>> = vec![vec![0 as usize; 0]; self.capacity.len() as usize];

        for i in 0 .. self.capacity.len() {
            for j in i .. self.capacity[i as usize].len() {
                if self.capacity[i as usize][j as usize] != 0 || self.capacity[j as usize][i as usize] != 0 {
                    adj_list[i as usize].push(j);
                    adj_list[j as usize].push(i);
                }
            }
        }
        let mut flow = vec![vec![0; self.capacity.len()]; self.capacity.len()];

        loop {
            let mut q: collections::VecDeque<usize> = collections::VecDeque::new();
            let mut parents = vec![None as Option<usize>; self.capacity.len()];
            let mut dists = vec![998244353 as i64; self.capacity.len()];
            let mut in_q = vec![false; self.capacity.len()];
            parents[source] = Some(source);
            q.push_back(source);
            in_q[source] = true;
            dists[source] = 0;

            while q.len() > 0 {
                let cur_node = *q.front().unwrap();
                q.pop_front();
                in_q[cur_node] = false;

                for next_node in adj_list[cur_node].clone().into_iter() {
                    if self.capacity[cur_node][next_node] - flow[cur_node][next_node] > 0
                        && dists[next_node] > dists[cur_node] + self.cost[cur_node][next_node] {
                            parents[next_node] = Some(cur_node);
                        dists[next_node] = dists[cur_node] + self.cost[cur_node][next_node];
                        if !in_q[next_node] {
                            q.push_back(next_node);
                            in_q[next_node] = true;
                        }
                    }
                }
            }

            if parents[sink].is_none() {
                break;
            }

            let mut amount: i64 = 998244600;
            let mut cur_node = sink;
            while cur_node != source {
                let mut prev_node = parents[cur_node].unwrap();
                amount = cmp::min(amount, self.capacity[prev_node][cur_node] - flow[prev_node][cur_node]);
                cur_node = prev_node;
            }

            let mut cur_node = sink;
            while cur_node != source {
                let mut prev_node = parents[cur_node].unwrap();
                flow[prev_node][cur_node] += amount;
                flow[cur_node][prev_node] -= amount;
                res += self.cost[prev_node][cur_node] * amount;
                cur_node = prev_node;
            }
        }

        

        res
    }
}

pub(crate) fn boj_11405() {
    let mut nm = read_vec::<i32>();
    let mut n = nm[0];
    let mut m = nm[1];

    // 0번 노드: 가상의 source node
    // 1 ~ n번 노드: 사람
    // n + 1 ~ n + m번 노드: 서점
    // n + m + 1번 노드: sink

    // 그래프에서 사람 번호 표현할 때는 +1
    // 서점 번호 표현할때는 + n + 1
    let mut graph = GraphForMCMF {
        capacity: vec![vec![0; (n + m + 2) as usize]; (n + m + 2) as usize],
        cost: vec![vec![998244353; (n + m + 2) as usize]; (n + m + 2) as usize]
    };

    let mut a = read_vec::<i64>();
    for i in 0 .. n {
        graph.capacity[(i + 1) as usize][(n + m + 1) as usize] = a[i as usize];
        graph.cost[(i + 1) as usize][(n + m + 1) as usize] = 0;
        // graph.capacity[(n + m + 1) as usize][(i + 1) as usize] = a[i as usize];
    }


    let mut b = read_vec::<i64>();
    for i in 0 .. m {
        graph.capacity[0][(i + n + 1) as usize] = b[i as usize];
        graph.cost[0][(i + n + 1) as usize] = 0;
        // graph.capacity[(i + n + 1) as usize][0] = b[i as usize];
    }

    for i in 1 .. n + 1 {
        for j in n + 1 .. n + m + 1 {
            // graph.capacity[i as usize][j as usize] = 998244353;
            graph.capacity[j as usize][i as usize] = 998244353;
        }
    }

    for i in 0 .. m {
        let mut c = read_vec::<i64>();
        for j in 0 .. n {
            graph.cost[(i + n + 1) as usize][(j + 1) as usize] = c[j as usize];
            graph.cost[(j + 1) as usize][(i + n + 1) as usize] = -c[j as usize];
        }
    }

    let ans = graph.get_mcmf(0 as usize, (n + m + 1) as usize);
    println!("{ans}");

}