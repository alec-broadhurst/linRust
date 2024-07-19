use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    values: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            let index = (row * self.cols) + col;
            Some(&self.values[index])
        } else {
            None
        }
    }

    pub fn add(&self, matrix_b: &Matrix<T>) -> Result<Matrix<T>, &str> {
        if self.rows != matrix_b.rows || self.cols != matrix_b.cols {
            return Err("Cannot add, matrices must have matching dimensions");
        }

        let mut new_values: Vec<T> = Vec::with_capacity(self.values.len());

        for i in 0..self.values.len() {
            new_values.push(self.values[i] + matrix_b.values[i]);
        }

        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            values: new_values,
        })
    }

    pub fn subtract(&self, matrix_b: &Matrix<T>) -> Result<Matrix<T>, &str> {
        if self.rows != matrix_b.rows || self.cols != matrix_b.cols {
            return Err("Cannot subtract, matrices must have matching dimensions");
        }

        let mut new_values: Vec<T> = Vec::with_capacity(self.values.len());

        for i in 0..self.values.len() {
            new_values.push(self.values[i] - matrix_b.values[i]);
        }

        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            values: new_values,
        })
    }

    pub fn transpose(&mut self) {
        let mut new_values: Vec<T> = Vec::with_capacity(self.rows * self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                new_values[j * self.rows + i] = self.values[i * self.cols + j];
            }
        }

        std::mem::swap(&mut self.rows, &mut self.cols);
        self.values = new_values;
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix;

    #[test]
    fn check_indexing() {
        let matrix: Matrix<u32> = Matrix {
            rows: 3,
            cols: 3,
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };

        assert_eq!(matrix.get(0, 0), Some(&1));
        assert_eq!(matrix.get(1, 1), Some(&5));
        assert_eq!(matrix.get(2, 0), Some(&7));

        assert_eq!(matrix.get(3, 0), None);
        assert_eq!(matrix.get(0, 3), None);
        assert_eq!(matrix.get(3, 3), None);
    }

    #[test]
    fn check_addition() {
        let matrix_a: Matrix<u32> = Matrix {
            rows: 2,
            cols: 2,
            values: vec![1, 2, 3, 4],
        };

        let matrix_b: Matrix<u32> = Matrix {
            rows: 2,
            cols: 2,
            values: vec![5, 6, 7, 8],
        };

        let expected_result: Matrix<u32> = Matrix {
            rows: 2,
            cols: 2,
            values: vec![6, 8, 10, 12],
        };

        match matrix_a.add(&matrix_b) {
            Ok(result) => assert_eq!(result, expected_result),
            Err(e) => panic!("Addition failed: {}", e),
        }
    }

    #[test]
    fn check_subtraction() {
        let matrix_a: Matrix<i32> = Matrix {
            rows: 3,
            cols: 3,
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };

        let matrix_b: Matrix<i32> = Matrix {
            rows: 3,
            cols: 3,
            values: vec![10, 21, 12, 13, 14, 15, 16, 17, 18],
        };

        let expected_result: Matrix<i32> = Matrix {
            rows: 3,
            cols: 3,
            values: vec![-9, -19, -9, -9, -9, -9, -9, -9, -9],
        };

        match matrix_a.subtract(&matrix_b) {
            Ok(result) => assert_eq!(result, expected_result),
            Err(e) => panic!("Addition failed: {}", e),
        }
    }

    #[test]
    fn check_transpose() {
        let mut matrix_a: Matrix<i32> = Matrix {
            rows: 2,
            cols: 3,
            values: vec![1, -3, 5, -9, 4, 7],
        };

        let expected_result: Matrix<i32> = Matrix {
            rows: 3,
            cols: 2,
            values: vec![1, 9, -3, 4, 5, 7],
        };

        matrix_a.transpose();
        assert_eq!(matrix_a, expected_result);
    }
}
