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

pub(crate) fn boj_5419() {
    let stdout = io::stdout(); // 2
    let mut out = io::BufWriter::new(stdout.lock());
    // task: for a given set of 2d points, find the number of ordered pair of points (p1, p2) where p1.x >= p2.x and p1.y <= p2.y
    // solution: use segment tree to count the number of points that have smaller y value than the current point
    let t = read_vec::<usize>()[0];

    for _ in 0..t {
        let n = read_vec::<usize>()[0];
        let mut points = Vec::new();
        for _ in 0..n {
            let xy = read_vec::<i64>();
            points.push(xy);
            // the inequality is reversed to make the problem easier
            points.last_mut().unwrap()[0] *= -1;
        }

        // value compression
        let mut compressed = vec![0 as i64; 2 * n];
        for i in 0..n {
            compressed[2 * i] = points[i][0];
            compressed[2 * i + 1] = points[i][1];
        }
        compressed.sort();
        // remove duplicates
        compressed.dedup();
        // update the points with compressed values
        for i in 0..n {
            points[i][0] = compressed.binary_search(&points[i][0]).unwrap() as i64;
            points[i][1] = compressed.binary_search(&points[i][1]).unwrap() as i64;
        }

        // make a segment tree
        // this will store y values of the scanned points
        let mut seg_tree = SegTree::new(2 * n, |a, b| a + b, 0);

        // sort by x value first, then by y value
        points.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        let mut ans = 0 as i64;
        for i in 0..n {
            let y = points[i][1];
            let mut cnt = seg_tree.query(0, y as usize + 1);
            ans += cnt as i64;
            seg_tree.update(y as usize, seg_tree.query_single(y as usize) + 1);
        }

        writeln!(out, "{}", ans).unwrap();
    }
}
