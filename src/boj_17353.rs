use std::{
    cmp::{max, min},
    io::{self, Write},
    mem::swap,
    str::FromStr,
};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn read<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

struct LazySegTree {
    n: usize,
    tree: Vec<i64>,
    // 초항, 공차
    lazy: Vec<(i64, i64)>,
}

impl LazySegTree {
    pub fn new(n: usize) -> Self {
        let mut tree_width = 1;
        while tree_width < n {
            tree_width *= 2;
        }
        let mut ret = Self {
            n: tree_width,
            tree: vec![0; tree_width * 2],
            lazy: vec![(0, 0); tree_width * 2],
        };
        ret
    }

    pub fn new_from_vec(v: Vec<i64>) -> Self {
        let n = v.len();
        let mut tree_width = 1;
        while tree_width < n {
            tree_width *= 2;
        }
        let mut ret = Self {
            n: tree_width,
            tree: vec![0; tree_width * 2],
            lazy: vec![(0, 0); tree_width * 2],
        };
        for i in 0..n {
            ret.tree[tree_width + i] = v[i];
        }
        for i in (0..tree_width).rev() {
            ret.tree[i] = ret.tree[i * 2] + ret.tree[i * 2 + 1];
        }
        ret
    }

    pub fn update(&mut self, l: usize, r: usize) {
        self.update_internal(1, 0, self.n, l, r, 1);
    }

    fn update_internal(
        &mut self,
        idx: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
        v: i64,
    ) {
        self.propagate(idx, start, end);
        if r <= start || end <= l {
            return;
        }
        if l <= start && end <= r {
            self.lazy[idx].0 += v;
            self.lazy[idx].1 += 1;
            self.propagate(idx, start, end);
            return;
        }
        let mid = (start + end) / 2;
        self.update_internal(idx * 2, start, mid, l, r, v);
        // count the number of elements in the left child
        let left_cnt = min(r, mid) as i64 - max(l, start) as i64;
        self.update_internal(idx * 2 + 1, mid, end, l, r, v + max(0, left_cnt) as i64);
        self.tree[idx] = self.tree[idx * 2] + self.tree[idx * 2 + 1];
    }

    fn propagate(&mut self, idx: usize, start: usize, end: usize) {
        if start + 1 < end {
            self.lazy[idx * 2].0 += self.lazy[idx].0;
            self.lazy[idx * 2].1 += self.lazy[idx].1;
            let mid = (start + end) / 2;
            self.lazy[idx * 2 + 1].0 +=
                self.lazy[idx].0 + self.lazy[idx].1 * ((mid - start) as i64);
            self.lazy[idx * 2 + 1].1 += self.lazy[idx].1;
        }
        self.tree[idx] += self.lazy[idx].0 * (end - start) as i64
            + self.lazy[idx].1 * (end - start) as i64 * (end - start - 1) as i64 / 2;
        self.lazy[idx].0 = 0;
        self.lazy[idx].1 = 0;
    }

    pub fn query(&mut self, l: usize, r: usize) -> i64 {
        self.query_internal(1, 0, self.n, l, r)
    }

    fn query_internal(&mut self, idx: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        self.propagate(idx, start, end);
        if r <= start || end <= l {
            return 0;
        }
        if l <= start && end <= r {
            return self.tree[idx];
        }
        let mid = (start + end) / 2;
        let mut ret = self.query_internal(idx * 2, start, mid, l, r);
        ret += self.query_internal(idx * 2 + 1, mid, end, l, r);
        return ret;
    }
}

pub(crate) fn boj_17353() {
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock()); // 3

    let n = read::<usize>();
    let a = read_vec::<i64>();
    let mut seg = LazySegTree::new_from_vec(a);
    let q = read::<usize>();
    for _ in 0..q {
        let v = read_vec::<usize>();
        if v[0] == 1 {
            let l = v[1] - 1;
            let r = v[2];
            seg.update(l, r);
        } else {
            let l = v[1] - 1;
            writeln!(out, "{}", seg.query(l, l + 1)).unwrap();
        }
    }
}
