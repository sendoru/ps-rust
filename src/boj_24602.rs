use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::io;
use std::mem::swap;

struct SparseTableForTree {
    n: usize,
    table: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl SparseTableForTree {
    fn new(arr: &Vec<usize>, depth: &Vec<usize>) -> Self {
        let n = arr.len();
        let mut table = vec![vec![0; 20]; n];
        for i in 0..n {
            table[i][0] = arr[i];
        }
        for j in 1..20 {
            for i in 0..n {
                table[i][j] = table[table[i][j - 1]][j - 1];
            }
        }
        Self {
            n,
            table,
            depth: depth.clone(),
        }
    }

    fn query(&self, n: usize, comp_cnt: usize) -> usize {
        let mut cur = n;
        for i in 0..20 {
            if comp_cnt & (1 << i) != 0 {
                cur = self.table[cur][i];
            }
        }
        cur
    }

    fn get_lca(&self, a: usize, b: usize) -> usize {
        let mut a = a;
        let mut b = b;
        if self.depth[a] > self.depth[b] {
            swap(&mut a, &mut b);
        }
        b = self.query(b, self.depth[b] - self.depth[a]);
        if a == b {
            return a;
        }
        for i in (0..20).rev() {
            if self.table[a][i] != self.table[b][i] {
                a = self.table[a][i];
                b = self.table[b][i];
            }
        }
        self.table[a][0]
    }

    fn get_dist(&self, a: usize, b: usize) -> usize {
        let lca = self.get_lca(a, b);
        self.depth[a] + self.depth[b] - 2 * self.depth[lca]
    }
}

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn get_parent_and_depth(adj_list: &Vec<Vec<usize>>, root: usize) -> (Vec<usize>, Vec<usize>) {
    let mut parent = vec![usize::MAX; adj_list.len()];
    let mut depth = vec![0; adj_list.len()];
    let mut q = VecDeque::new();
    q.push_back(root);
    parent[root] = root;
    depth[root] = 0;
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        for nxt in &adj_list[cur] {
            if parent[*nxt] == usize::MAX {
                parent[*nxt] = cur;
                depth[*nxt] = depth[cur] + 1;
                q.push_back(*nxt);
            }
        }
    }

    (parent, depth)
}

pub(crate) fn boj_24602() {
    let t = read_vec::<usize>()[0];
    for _ in 0..t {
        let n = read_vec::<usize>()[0];
        let mut adj_list = vec![vec![]; n];
        for _ in 0..n - 1 {
            let edge = read_vec::<usize>();
            let s = edge[0] - 1;
            let e = edge[1] - 1;
            adj_list[s].push(e);
            adj_list[e].push(s);
        }
        let (parent, depth) = get_parent_and_depth(&adj_list, 0);
        let mut sparse_table = SparseTableForTree::new(&parent, &depth);

        let mut perm = vec![];
        for i in 0..n {
            let cur = read_vec::<usize>()[0] - 1;
            perm.push(cur);
        }

        let mut ans = true;
        for i in 0..n - 1 {
            let dist = sparse_table.get_dist(perm[i], perm[i + 1]);
            if dist > 3 {
                ans = false;
                break;
            }
        }

        if ans {
            println!("1");
        } else {
            println!("0");
        }
    }
}
