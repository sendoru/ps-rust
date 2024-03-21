use std::io;
use std::str::FromStr;
use std::collections::vec_deque::VecDeque as Deque;
use std::io::Write;
use std::io::BufRead;

fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

fn read_vec_locked<T, R: BufRead> (stdin: &mut R) -> Vec<T> where 
T: FromStr,
<T as FromStr>::Err: std::fmt::Debug
{
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
  
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        let mut parent = vec![0 as usize; size];
        for i in 0 .. size {
            parent[i] = i;
        }
        Self {parent: parent, size: vec![1 as usize; size]}
    }

    pub fn get_parent(&mut self, idx: usize) -> usize {
        if self.parent[idx] == idx {
            return idx;
        }
        
        self.parent[idx] = self.get_parent(self.parent[idx]);
        return self.parent[idx];
    }

    pub fn get_size(&mut self, idx: usize) -> usize {
        let idx_new = self.get_parent(idx);
        return self.size[idx_new];
    }
    
    /* Merge elem ```a``` and elem ```b``` 
     */
    pub fn unite (&mut self, a: usize, b: usize) {
        let a_par = self.get_parent(a);
        let b_par = self.get_parent(b);
        if a_par == b_par {
            return;
        }

        if a_par < b_par {
            self.size[a_par] += self.size[b_par];
            self.parent[b_par] = self.parent[a_par];
        }
        else {
            self.size[b_par] += self.size[a_par];
            self.parent[a_par] = self.parent[b_par];
        }
    }
}

fn bfs(adj_list: &Vec::<Vec::<usize>>) -> Vec<Vec<usize>> {
    let mut directed_adj_list: Vec<Vec<usize>> = vec![vec![]; adj_list.len()];
    let mut q: Deque<usize> = Deque::new();
    let mut visited: Vec<bool> = vec![false; adj_list.len()];
    q.push_back(0);
    visited[0] = true;
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        for &next in &adj_list[cur] {
            if visited[next] {
                continue;
            }
            visited[next] = true;
            directed_adj_list[cur].push(next);
            q.push_back(next);
        }
    }
    return directed_adj_list;
}

pub(crate) fn boj_18267() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let stdin: io::Stdin = std::io::stdin();
    let mut input: io::BufReader<io::StdinLock<'_>> = std::io::BufReader::new(stdin.lock());

    let nm = read_vec_locked::<usize, io::BufReader<io::StdinLock<'_>>>(&mut input);
    let n = nm[0];
    let m = nm[1];
    let mut s = read_vec::<String>()[0].clone();
    
    let mut adj_list = vec![vec![]; n];
    for i in 0..n-1 {
        let mut uv = read_vec_locked::<usize, io::BufReader<io::StdinLock<'_>>>(&mut input);
        uv[0] -= 1;
        uv[1] -= 1;
        adj_list[uv[0]].push(uv[1]);
        adj_list[uv[1]].push(uv[0]);
    }

    let directed_adj_list = bfs(&adj_list);
    // 정점 분할 해야될듯
    // 일단 각 정점 i에 대해, 부모랑 연결되는 간선은 2*i, 자식과 연결되는 간선은 2*i+1번 정점에 연결시키고

    let mut dsu_disconnect_h = Dsu::new(2*n);
    let mut dsu_disconnect_g = Dsu::new(2*n);
    for parent in 0..n {
        for child in &directed_adj_list[parent] {
            dsu_disconnect_h.unite(2 * parent + 1, 2 * child);
            dsu_disconnect_g.unite(2 * parent + 1, 2 * child);
        }
    }

    for i in 0..n {
        if s.chars().nth(i).unwrap() == 'H' {
            dsu_disconnect_g.unite(2 * i, 2 * i + 1);
        }
        else {
            dsu_disconnect_h.unite(2 * i, 2 * i + 1);
        }
    }

    let mut ans = vec![0;m];

    for i in 0..m {
        let abc = read_vec_locked::<char, io::BufReader<io::StdinLock<'_>>>(&mut input);
        let a = abc[0] as usize - '0' as usize - 1;
        let b = abc[1] as usize - '0' as usize - 1;
        let c = abc[2];
        match c {
            'H' => {
                if dsu_disconnect_h.get_parent(2 * a) != dsu_disconnect_h.get_parent(2 * b) ||
                dsu_disconnect_h.get_parent(2 * a) != dsu_disconnect_h.get_parent(2 * b + 1) ||
                dsu_disconnect_h.get_parent(2 * a + 1) != dsu_disconnect_h.get_parent(2 * b) ||
                dsu_disconnect_h.get_parent(2 * a + 1) != dsu_disconnect_h.get_parent(2 * b + 1) {
                    ans[i] = 1;
                }
                else {
                    ans[i] = 0;
                }
            }
            'G' => {
                if dsu_disconnect_g.get_parent(2 * a) != dsu_disconnect_g.get_parent(2 * b) || 
                dsu_disconnect_g.get_parent(2 * a + 1) != dsu_disconnect_g.get_parent(2 * b) || 
                dsu_disconnect_g.get_parent(2 * a) != dsu_disconnect_g.get_parent(2 * b + 1) || 
                dsu_disconnect_g.get_parent(2 * a + 1) != dsu_disconnect_g.get_parent(2 * b + 1){
                    ans[i] = 1;
                }
                else {
                    ans[i] = 0;
                }
            }
            _ => {}
        }
    }

    for i in ans {
        write!(out, "{}", i).unwrap();
    }

}