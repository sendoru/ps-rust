use std::io::Write;
use std::{cmp, collections::VecDeque, io, ptr::read, str::FromStr};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn modpow(b: i64, e: i64, m: i64) -> i64 {
    if e == 0 {
        return 1;
    }
    let mut half = modpow(b, e / 2, m);
    half = (half * half) % m;
    if e % 2 == 1 {
        half = (half * b) % m;
    }
    return half;
}

fn modinv(a: i64, m: i64) -> i64 {
    return modpow(a, m - 2, m);
}

pub(crate) fn boj_25569() {
    let MOD = 1_000_000_007;
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    // factorial precalc
    let mut fact = vec![1; 800_001];
    let mut fact_inv = vec![1; 800_001];
    for i in 1..800_001 {
        fact[i] = (fact[i - 1] * i as i64) % MOD;
        fact_inv[i] = fact_inv[i - 1] * modinv(i as i64, MOD) % MOD;
    }

    let n = read_vec::<usize>()[0];
    let mut ans = 1;
    for _ in 0..n {
        let ab = read_vec::<i64>();
        let c = ab[0] + ab[1];
        // cCa - 1 = c! / (c - a)! / a! - 1
        ans *= ((fact[c as usize] * fact_inv[ab[0] as usize] % MOD * fact_inv[ab[1] as usize]
            % MOD)
            + (MOD - 1))
            % MOD;
        ans %= MOD;
    }

    writeln!(out, "{}", ans % MOD).unwrap();
}
