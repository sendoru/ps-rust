use std::{cmp, io, ptr::read, str::FromStr, io::Write};


fn read_vec<T>() -> Vec<T> where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| { s.parse().unwrap() })
        .collect()
}

pub struct Dsu {
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

    pub fn has_same_parent(&mut self, a: usize, b: usize) -> bool {
        let a_parent = self.get_parent(a);
        let b_parent = self.get_parent(b);
        return a_parent == b_parent;
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
            self.size[a_par] = cmp::max(self.size[a_par], self.size[b_par]);
            self.parent[b_par] = self.parent[a_par];
        }
        else {
            self.size[b_par] = cmp::max(self.size[a_par], self.size[b_par]);
            self.parent[a_par] = self.parent[b_par];
        }
    }
}

pub(crate) fn boj_17619() {
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock()); // 3

    let nq = read_vec::<i32>();
    let n = nq[0];
    let q = nq[1];
    
    let mut logs = vec![vec![0, 0, 0]; n as usize];
    for i in 0 .. n {
        logs[i as usize] = read_vec::<i32>();
        // 인덱스 저장
        logs[i as usize].push(i + 1);
    }

    let mut dsu = Dsu::new((n + 1) as usize);
    for i in 1 .. n + 1 {
        dsu.size[i as usize] = logs[(i - 1) as usize][1] as usize;
    }

    // 근데 y좌표를 왜 신경쓰지?

    // 일단 스타팅 x좌표깆누으로 정렬해볼까
    
    logs.sort();
    for i in 0 as usize .. (n - 1) as usize {
        // i + 1번쨰 스타팅이 i번째 엔딩보다 앞에있으면 연결됨
        if logs[i + 1][0] <= dsu.get_size(logs[i][3] as usize) as i32 {
            dsu.unite(logs[i + 1][3] as usize, logs[i][3] as usize);
        }
    }

    for i in 0 .. q {
        let query = read_vec::<i32>();
        if dsu.has_same_parent(query[0] as usize, query[1] as usize) {
            writeln!(out, "1", ); // 4
        }
        else {
            writeln!(out, "0", ); // 4
        }
    }
}