use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix {
    pub rows: Vec<Vec<i32>>
}

impl Matrix {
    pub fn new(num_rows: usize, num_columns: usize) -> Matrix {
        let mut rows = Vec::with_capacity(num_rows);
        for _ in 0..num_rows {
            rows.push(vec![0; num_columns]);
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
        let src_elem = self.rows[src_row][col];
        let dest_elem = self.rows[dest_row][col];
        println!("R{dest_row} = {src_elem} * R{dest_row} - {dest_elem} * R{src_row}");
        for col in 0..self.rows[0].len() {
            self.rows[dest_row][col] = self.rows[dest_row][col] * src_elem - self.rows[src_row][col] * dest_elem;
        }
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<i32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}
