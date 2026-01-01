use std::{fmt::Display, ops::{Index, IndexMut}};
use num::Integer;

#[derive(Debug)]
pub struct Matrix<T> {
    pub rows: Vec<Vec<T>>
}

impl<T: Integer+Clone+Copy+Display> Matrix<T> {
    pub fn new(num_rows: usize, num_columns: usize) -> Matrix<T> {
        let mut rows = Vec::with_capacity(num_rows);
        for _ in 0..num_rows {
            rows.push(vec![T::zero(); num_columns]);
        }
        Matrix{rows}
    }

    pub fn print(&self) {
        for row in &self.rows {
            for col in row {
                print!(" {col:5}")
            }
            println!();
        }
        println!();
    }

    pub fn swap(&mut self, first: usize, second: usize) {
        self.rows.swap(first, second);
    }

    pub fn subtract_rows(&mut self, src_row: usize, dest_row: usize, col: usize) {
        use num::integer::lcm;
        let src_elem = self.rows[src_row][col];
        let dest_elem = self.rows[dest_row][col];
        let common = lcm(src_elem, dest_elem);
        let src_mul = common / src_elem;
        let dest_mul = common / dest_elem;

        println!("R{dest_row} = {dest_mul} * R{dest_row} - {src_mul} * R{src_row}");
        for col in 0..self.rows[0].len() {
            self.rows[dest_row][col] = self.rows[dest_row][col] * dest_mul - self.rows[src_row][col] * src_mul;
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}
