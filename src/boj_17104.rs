use std::{
    cmp::{self, max, min},
    io,
    ptr::read,
    str::FromStr,
};

// impl clone
#[derive(Clone, Copy)]
struct Complex64 {
    re: f64,
    im: f64,
}

impl Complex64 {
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
    fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
    fn norm(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    fn abs(&self) -> f64 {
        self.norm().sqrt()
    }
}

impl std::ops::Add for Complex64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub for Complex64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl std::ops::Div for Complex64 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn fft(a: &Vec<Complex64>, rev: bool) -> Vec<Complex64> {
    let mut n = 1;
    let mut log2n = 0;
    while n < a.len() {
        n *= 2;
        log2n += 1;
    }
    let mut ret = vec![Complex64::new(0.0, 0.0); n];
    for i in 0..a.len() {
        ret[i] = a[i];
    }
    // radix-2 Cooley-Tukey FFT
    for i in 0..n {
        let mut j = 0;
        for k in 0..log2n {
            j = j << 1;
            j |= (i >> k) & 1;
        }
        if i < j {
            ret.swap(i, j);
        }
    }

    let mut bucket_size = 2;
    while bucket_size <= n {
        let theta = 2.0 * std::f64::consts::PI / bucket_size as f64;
        let mut w = Complex64::new((theta).cos(), (theta).sin() * 1.0);
        if rev {
            w = w.conj();
        }
        for i in (0..n).step_by(bucket_size) {
            let mut omega = Complex64::new(1.0, 0.0);
            for j in 0..bucket_size / 2 {
                let x = ret[i + j];
                let y = omega * ret[i + j + bucket_size / 2];
                ret[i + j] = x + y;
                ret[i + j + bucket_size / 2] = x - y;
                omega = omega * w;
            }
        }
        bucket_size *= 2;
    }

    if rev {
        for i in 0..n {
            ret[i] = ret[i] / Complex64::new(n as f64, 0.);
        }
    }
    ret
}

fn convolve(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let mut n = 1;
    let mut log2n = 0;
    while n < a.len() + b.len() {
        n *= 2;
        log2n += 1;
    }
    // n *= 2;
    let mut ab_cpx = vec![Complex64::new(0.0, 0.0); n];
    for i in 0..a.len() {
        ab_cpx[i].re = a[i];
    }
    for i in 0..b.len() {
        ab_cpx[i].im = b[i];
    }
    let mut ab = fft(&ab_cpx, false);
    for i in 0..=n / 2 {
        let x = ab[i];
        let y = ab[(n - i) % n];
        ab[i] = (x * x - y.conj() * y.conj()) * Complex64::new(0.0, -0.25);
        ab[(n - i) % n] = (y * y - x.conj() * x.conj()) * Complex64::new(0.0, -0.25);
    }
    // ret.resize(a.len(), 0.0);
    let ret_cpx = fft(&ab, true);
    let mut ret = vec![0.0; ret_cpx.len()];
    for i in 0..ret_cpx.len() {
        ret[i] = ret_cpx[i].re;
    }
    ret
}

fn eratosthenes(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        if !is_prime[i] {
            continue;
        }
        let mut j = 2;
        while i * j <= n {
            is_prime[i * j] = false;
            j += 1;
        }
    }
    is_prime
}

pub(crate) fn boj_17104() {
    let n = read_vec::<usize>()[0];
    let mut is_prime = eratosthenes(1_000_000);
    let is_prime_float: Vec<f64> = is_prime.iter().map(|&x| x as i32 as f64).collect();
    let mut is_prime_compressed = vec![0.0; 500_000 + 1];
    let mut idx1 = 0;
    let mut idx2 = 1;
    while idx2 <= 1_000_000 {
        is_prime_compressed[idx1] = is_prime_float[idx2];
        idx1 += 1;
        idx2 += 2;
    }
    let mut partition_cnt = convolve(&is_prime_compressed, &is_prime_compressed);
    for _ in 0..n {
        let x = read_vec::<usize>()[0];
        if x == 4 {
            println!("1");
            continue;
        }
        println!("{}", (partition_cnt[(x - 1) / 2].round() as usize + 1) / 2);
    }
}
