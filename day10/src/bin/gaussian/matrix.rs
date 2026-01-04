use std::{fmt::Display, ops::{Index, IndexMut, Range}};
use num::Integer;

#[derive(Debug)]
pub struct Matrix<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T: Integer+Clone+Copy+Display> Matrix<T> {
    pub fn new(num_rows: usize, num_columns: usize) -> Matrix<T> {
        let mut rows = Vec::with_capacity(num_rows);
        for _ in 0..num_rows {
            rows.push(vec![T::zero(); num_columns]);
        }
        Matrix{rows}
    }

    #[allow(dead_code)]
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

        // println!("R{dest_row} = {dest_mul} * R{dest_row} - {src_mul} * R{src_row}");
        for col in 0..self.rows[0].len() {
            self.rows[dest_row][col] = self.rows[dest_row][col] * dest_mul - self.rows[src_row][col] * src_mul;
        }
    }

    #[allow(dead_code)]
    pub fn rotate_column_left(&mut self, range: Range<usize>) {
        for row in self.rows.iter_mut() {
            row[range.clone()].rotate_left(1);
        }
    }

    // Do Gaussian elimination to get the matrix as close as possible to
    // reduced row echelon form as possible.  Note that since we're using
    // integer coefficients, the leading coefficient in a row might not
    // be 1 (and it might be negative).
    pub fn reduce(&mut self) -> Vec<usize> {
        let num_rows = self.rows.len();
        let num_vars = self.rows[0].len() - 1;
        let zero = T::zero();
        let mut free_columns = Vec::<usize>::new();

        // Do Gaussian elimination (or similar) to come up with a
        // solution (or family of solutions).  Start with row
        // echelon form.
        let mut arranged_rows = 0;
        for col in 0..num_vars {
            let mut found_pivot = false;

            // Find a row below `arranged_rows` with a non-zero entry in column `col`.
            for row in arranged_rows..num_rows {
                if self[row][col] != zero {
                    if row > arranged_rows {
                        // Move row `row` up to arranged_rows
                        self.swap(row, arranged_rows);
                    }
                    arranged_rows += 1;
                    
                    // If any rows below `arranged_rows`` have a non-zero entry in row `col`,
                    // use row subtraction to change that column to zero.
                    for row in arranged_rows..num_rows {
                        if self[row][col] != zero {
                            self.subtract_rows(arranged_rows - 1, row, col);
                        }
                    }

                    // We are done with the current column.
                    found_pivot = true;
                    break;
                }
            }
            if !found_pivot {
                free_columns.push(col);
            }
        }

        // Try to get close to reduced row echelon form.  For each column
        // that is uniquely determined, substitute into rows above.
        for row in (0..num_rows).rev() {
            // Find the first non-zero column in this row
            for col in 0..num_vars {
                if self[row][col] != zero {
                    // TODO: Should we try to divide out any common factor for this row?
                    // Eliminate non-zero values above this entry
                    for r in 0..row {
                        if self[r][col] != zero {
                            self.subtract_rows(row, r, col);
                        }
                    }
                    break;
                }
            }
        }

        // Remove rows of all zeroes
        self.rows.retain(|row| row.iter().any(|&v| v != zero));

        free_columns
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
