use std::io::Write; // Import the Write trait
use std::{str::FromStr, *};

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
