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


pub(crate) fn boj_18267() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let nm = read_vec::<usize>();
    let n = nm[0];
    let m = nm[1];
    let mut s = read_vec::<String>()[0].clone();
    
    let mut adj_list = vec![vec![]; n];
    for i in 0..n-1 {
        let mut uv = read_vec::<usize>();
        uv[0] -= 1;
        uv[1] -= 1;
        adj_list[uv[0]].push([uv[1], i + n]);
        adj_list[uv[1]].push([uv[0], i + n]);
    }

    let mut dsu_disconnected_h = Dsu::new(2 * n - 1);
    let mut dsu_disconnected_g = Dsu::new(2 * n - 1);
    let s_chars: Vec<char> = s.chars().collect();

    for i in 0..n {
        match s_chars[i] {
            'H' => {
                for &next in &adj_list[i] {
                    dsu_disconnected_g.unite(i, next[1]);
                }
            }
            'G' => {
                for &next in &adj_list[i] {
                    dsu_disconnected_h.unite(i, next[1]);
                }
            }
            _ => {}
        }
    }

    let mut ans = vec![0;m];

    for i in 0..m {
        let abc = read_vec::<String>();
        let a = abc[0].parse::<usize>().unwrap() - 1 as usize;
        let b = abc[1].parse::<usize>().unwrap() - 1 as usize;
        let c = abc[2].chars().next().unwrap();
        match c {
            'H' => {
                if a == b && s_chars[a] == 'H' {
                    ans[i] = 1;
                    continue;
                }
                else if (dsu_disconnected_h.get_parent(a) != dsu_disconnected_h.get_parent(b)) {
                    ans[i] = 1;
                }
            }
            'G' => {
                if a == b && s_chars[a] == 'G' {
                    ans[i] = 1;
                    continue;
                }
                else if (dsu_disconnected_g.get_parent(a) != dsu_disconnected_g.get_parent(b)) {
                    ans[i] = 1;
                }
            }
            _ => {}
        }
    }

    for i in ans {
        write!(out, "{}", i).unwrap();
    }

}