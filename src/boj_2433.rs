use std::io::Write; // Import the Write trait
use std::{str::FromStr, *};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

struct SegTree<T>
where
    T: std::clone::Clone,
{
    n: usize,
    tree: Vec<T>,
    f: fn(T, T) -> T,
    unit: T,
}

impl<T> SegTree<T>
where
    T: std::clone::Clone,
{
    // initialize with n id elements, f function, and unit value
    fn new(n: usize, f: fn(T, T) -> T, unit: T) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        Self {
            n: n_,
            tree: vec![unit.clone(); 2 * n_],
            f,
            unit,
        }
    }

    // initialize with given elements, f function, and unit value
    fn new_with(v: &Vec<T>, f: fn(T, T) -> T, unit: T) -> Self {
        let mut n = 1;
        while n < v.len() {
            n *= 2;
        }
        let mut tree = vec![unit.clone(); 2 * n]; // Clone `unit` when initializing the `tree` vector
        tree[n..n + v.len()].clone_from_slice(&v);
        for i in (1..n).rev() {
            tree[i] = (f)(tree[i * 2].clone(), tree[i * 2 + 1].clone());
        }
        Self { n, tree, f, unit }
    }

    fn update(&mut self, i: usize, v: T) {
        let mut i = i + self.n;
        self.tree[i] = v;
        while i > 1 {
            i /= 2;
            self.tree[i] = (self.f)(self.tree[i * 2].clone(), self.tree[i * 2 + 1].clone());
        }
    }

    // query for single element
    fn query_single(&self, i: usize) -> T {
        self.tree[i + self.n].clone()
    }

    // query for SEMI-OPEN interval [l, r)
    fn query(&self, l: usize, r: usize) -> T {
        let mut l = l + self.n;
        let mut r = r + self.n;
        let mut ret = self.unit.clone();
        while l < r {
            if l % 2 == 1 {
                ret = (self.f)(ret.clone(), self.tree[l].clone());
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                ret = (self.f)(ret.clone(), self.tree[r].clone());
            }
            l /= 2;
            r /= 2;
        }
        ret.clone()
    }
}

struct SegTree2D<T>
where
    T: std::clone::Clone,
{
    n: usize,
    m: usize,
    tree: Vec<T>,
    f: fn(T, T) -> T,
    unit: T,
}

impl<T> SegTree2D<T>
where
    T: std::clone::Clone,
{
    // initialize with n*m id elements, f function, and unit value
    fn new(n: usize, m: usize, f: fn(T, T) -> T, unit: T) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        let mut m_ = 1;
        while m_ < m {
            m_ *= 2;
        }
        Self {
            n: n_,
            m: m_,
            tree: vec![unit.clone(); 2 * n_ * m_],
            f,
            unit,
        }
    }

    // initialize with given elements, f function, and unit value
    fn new_with(v: &Vec<Vec<T>>, f: fn(T, T) -> T, unit: T) -> Self {
        let mut n = 1;
        while n < v.len() {
            n *= 2;
        }
        let mut m = 1;
        while m < v[0].len() {
            m *= 2;
        }
        let mut tree = vec![unit.clone(); 2 * n * m]; // Clone `unit` when initializing the `tree` vector
        for i in 0..v.len() {
            tree[n * (i + 1)..n * (i + 1) + v[i].len()].clone_from_slice(&v[i]);
        }
        for i in (1..n).rev() {
            for j in 0..m {
                tree[i * m + j] = (f)(tree[i * 2 * m + j].clone(), tree[i * 2 * m + j + 1].clone());
            }
        }
        for j in (1..m).rev() {
            for i in 0..n {
                tree[i * m + j] = (f)(tree[i * m * 2 + j].clone(), tree[i * m * 2 + j + 1].clone());
            }
        }
        Self {
            n,
            m,
            tree,
            f,
            unit,
        }
    }

    // update for single element
    fn update(&mut self, i: usize, j: usize, v: T) {
        let mut i = i + self.n;
        let mut j = j + self.m;
        self.tree[i * self.m + j] = v;
        while i > 1 {
            i /= 2;
            self.tree[i * self.m + j] = (self.f)(
                self.tree[i * 2 * self.m + j].clone(),
                self.tree[i * 2 * self.m + j + 1].clone(),
            );
        }
        while j > 1 {
            j /= 2;
            self.tree[i * self.m + j] = (self.f)(
                self.tree[i * self.m * 2 + j].clone(),
                self.tree[i * self.m * 2 + j + 1].clone(),
            );
        }
    }

    // query for single element
    fn query_single(&self, i: usize, j: usize) -> T {
        self.tree[i * self.m + j].clone()
    }

    // query for SEMI-OPEN interval [x1, x2) x [y1, y2)
    fn query(&self, x1: usize, x2: usize, y1: usize, y2: usize) -> T {
        let mut x1 = x1 + self.n;
        let mut x2 = x2 + self.n;
        let mut y1 = y1 + self.m;
        let mut y2 = y2 + self.m;
        let mut ret = self.unit.clone();
        while x1 < x2 {
            if x1 % 2 == 1 {
                let mut y1_ = y1;
                let mut y2_ = y2;
                while y1_ < y2_ {
                    if y1_ % 2 == 1 {
                        ret = (self.f)(ret.clone(), self.tree[x1 * self.m + y1_].clone());
                        y1_ += 1;
                    }
                    if y2_ % 2 == 1 {
                        y2_ -= 1;
                        ret = (self.f)(ret.clone(), self.tree[x1 * self.m + y2_].clone());
                    }
                    y1_ /= 2;
                    y2_ /= 2;
                }
                x1 += 1;
            }
            if x2 % 2 == 1 {
                x2 -= 1;
                let mut y1_ = y1;
                let mut y2_ = y2;
                while y1_ < y2_ {
                    if y1_ % 2 == 1 {
                        ret = (self.f)(ret.clone(), self.tree[x2 * self.m + y1_].clone());
                        y1_ += 1;
                    }
                    if y2_ % 2 == 1 {
                        y2_ -= 1;
                        ret = (self.f)(ret.clone(), self.tree[x2 * self.m + y2_].clone());
                    }
                    y1_ /= 2;
                    y2_ /= 2;
                }
            }
            x1 /= 2;
            x2 /= 2;
        }
        ret.clone()
    }
}

// @param
// n: size of the array
// tree: segment tree
// lazy: lazy propagation tree
// f: function to merge two elements T
// g: function to apply lazy value U to T
// h: function to merge two lazy values U
// unit: unit value of T
struct LazySegTree<T, U>
where
    T: std::clone::Clone, T: Copy,
    U: std::clone::Clone, U: std::cmp::Eq, U: Copy,
{
    n: usize,
    tree: Vec<T>,
    lazy: Vec<U>,
    f: fn(T, T) -> T,
    g: fn(T, U) -> T,
    h: fn(U, U) -> U,
    unit: T,
    unit_lazy: U,
}
impl<T, U> LazySegTree<T, U>
where
    T: std::clone::Clone, T: Copy,
    U: std::clone::Clone, U: std::cmp::Eq, U: Copy,
{
    // initialize with n id elements, f function, g function, h function, unit value, and unit_lazy value
    fn new(n: usize, f: fn(T, T) -> T, g: fn(T, U) -> T, h: fn(U, U) -> U, unit: T, unit_lazy: U) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        Self {
            n: n_,
            tree: vec![unit.clone(); 2 * n_],
            lazy: vec![unit_lazy.clone(); 2 * n_],
            f,
            g,
            h,
            unit,
            unit_lazy,
        }
    }

    // initialize with given elements, f function, g function, h function, unit value, and unit_lazy value
    fn new_with(v: &Vec<T>, f: fn(T, T) -> T, g: fn(T, U) -> T, h: fn(U, U) -> U, unit: T, unit_lazy: U) -> Self {
        let mut n = 1;
        while n < v.len() {
            n *= 2;
        }
        let mut tree = vec![unit.clone(); 2 * n]; // Clone `unit` when initializing the `tree` vector
        tree[n..n + v.len()].clone_from_slice(&v);
        for i in (1..n).rev() {
            tree[i] = (f)(tree[i * 2].clone(), tree[i * 2 + 1].clone());
        }
        Self {
            n,
            tree,
            lazy: vec![unit_lazy.clone(); 2 * n],
            f,
            g,
            h,
            unit,
            unit_lazy,
        }
    }

    fn eval(&mut self, k: usize) {
        if self.lazy[k] != self.unit_lazy {
            if k < self.n {
                self.lazy[k * 2] = (self.h)(self.lazy[k * 2].clone(), self.lazy[k].clone());
                self.lazy[k * 2 + 1] = (self.h)(self.lazy[k * 2 + 1].clone(), self.lazy[k * 2].clone());
            }
            self.tree[k] = (self.g)(self.tree[k].clone(), self.lazy[k].clone());
            self.lazy[k] = self.unit_lazy.clone();
        }
    }

    fn update(&mut self, a: usize, b: usize, x: U) {
        self.update_lazy(a, b, x, 1, 0, self.n);
    }

    fn update_lazy(&mut self, a: usize, b: usize, x: U, k: usize, l: usize, r: usize) {
        self.eval(k);
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.lazy[k] = (self.h)(self.lazy[k].clone(), x);
            self.eval(k);
        } else {
            self.update_lazy(a, b, x, k * 2, l, (l + r) / 2);
            self.update_lazy(a, b, x, k * 2 + 1, (l + r) / 2, r);
            self.tree[k] = (self.f)(self.tree[k * 2].clone(), self.tree[k * 2 + 1].clone());
        }
    }

    fn query(&mut self, a: usize, b: usize) -> T {
        self.query_lazy(a, b, 1, 0, self.n)
    }

    fn query_lazy(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        self.eval(k);
        if b <= l || r <= a {
            return self.unit.clone();
        }
        if a <= l && r <= b {
            return self.tree[k].clone();
        }
        let vl = self.query_lazy(a, b, k * 2, l, (l + r) / 2);
        let vr = self.query_lazy(a, b, k * 2 + 1, (l + r) / 2, r);
        (self.f)(vl, vr)
    }

    fn update_single(&mut self, i: usize, v: T) {
        let mut i = i + self.n;
        self.tree[i] = v;
        while i > 1 {
            i /= 2;
            self.tree[i] = (self.f)(self.tree[i * 2].clone(), self.tree[i * 2 + 1].clone());
        }
    }
}

pub(crate) fn boj_2433() {
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock()); // 3

    let nmc = read_vec::<usize>();
    let v = read_vec::<i64>();
    let mut min_segtree = SegTree::new_with(&v, |a, b| a.min(b), i64::MAX);
    let mut max_segtree = SegTree::new_with(&v, |a, b| a.max(b), i64::MIN);

    let mut printed = false;
    // max(a[i . . . i+m−1])−min(a[i . . . i+m−1]) ≤ c를 만족하는 모든 i를 오름차순으로 한 줄에 하나씩 출력한다.
    for i in 0..nmc[0] - nmc[1] + 1 {
        let min = min_segtree.query(i, i + nmc[1]);
        let max = max_segtree.query(i, i + nmc[1]);
        if max - min <= nmc[2] as i64 {
            writeln!(out, "{}", i + 1).unwrap(); // Use the writeln! macro to write to the BufWriter
            printed = true;
        }
    }

    if !printed {
        writeln!(out, "NONE").unwrap();
    }
}
