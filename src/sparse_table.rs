use std::*;

mod sparse_table {
    pub struct SparseTable<T> {
        table: Vec<Vec<T>>,
    }
}

impl<T> SparseTable<T>
where
    T: Copy + Default + std::ops::Add<Output = T>,
{
    pub fn new(arr: &[T]) -> Self {
        let n = arr.len();
        let mut table = vec![vec![T::default(); n]; (n as f64).log2() as usize + 1];

        for i in 0..n {
            table[0][i] = arr[i];
        }

        for k in 1..table.len() {
            for i in 0..=(n - (1 << k)) {
                table[k][i] = table[k - 1][i] + table[k - 1][i + (1 << (k - 1))];
            }
        }

        SparseTable { table }
    }

    pub fn query(&self, l: usize, r: usize) -> T {
        let k = ((r - l + 1) as f64).log2() as usize;
        self.table[k][l] + self.table[k][r - (1 << k) + 1]
    }
}
