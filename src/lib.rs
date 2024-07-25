use std::ops::{Add, AddAssign, Mul, Sub};

mod error;
use error::MatrixError;

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    values: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Default + AddAssign,
{
    pub fn new(rows: usize, cols: usize, values: Vec<T>) -> Matrix<T> {
        Matrix {
            rows: rows,
            cols: cols,
            values: values,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            let index = (row * self.cols) + col;
            Some(&self.values[index])
        } else {
            None
        }
    }

    pub fn add(&self, matrix_b: &Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        if self.rows != matrix_b.rows || self.cols != matrix_b.cols {
            return Err(MatrixError::DimensionMismatch(format!(
                "Cannot add matricies of dimensions {}x{} and {}x{}",
                self.rows, self.cols, matrix_b.rows, matrix_b.cols
            )));
        }

        let mut new_values: Vec<T> = Vec::with_capacity(self.values.len());

        for i in 0..self.values.len() {
            new_values.push(self.values[i] + matrix_b.values[i]);
        }

        Ok(Matrix::new(self.rows, self.cols, new_values))
    }

    pub fn subtract(&self, matrix_b: &Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        if self.rows != matrix_b.rows || self.cols != matrix_b.cols {
            return Err(MatrixError::DimensionMismatch(format!(
                "Cannot subtract matricies of dimensions {}x{} and {}x{}",
                self.rows, self.cols, matrix_b.rows, matrix_b.cols
            )));
        }

        let mut new_values: Vec<T> = Vec::with_capacity(self.values.len());

        for i in 0..self.values.len() {
            new_values.push(self.values[i] - matrix_b.values[i]);
        }

        Ok(Matrix::new(self.rows, self.cols, new_values))
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut new_values: Vec<T> = Vec::with_capacity(self.rows * self.cols);

        for i in 0..self.cols {
            for j in 0..self.rows {
                new_values.push(*self.get(j, i).unwrap());
            }
        }

        Matrix::new(self.cols, self.rows, new_values)
    }

    pub fn mult_naive(&self, matrix_b: &Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        if self.cols != matrix_b.rows {
            return Err(MatrixError::DimensionMismatch(format!(
                "Cannot multiply  matricies of dimensions {}x{} and {}x{}",
                self.rows, self.cols, matrix_b.rows, matrix_b.cols
            )));
        }

        let b_t = matrix_b.transpose();
        let mut new_values: Vec<T> = Vec::with_capacity(matrix_b.rows * self.cols);

        for i in 0..self.rows {
            for j in 0..matrix_b.cols {
                let mut sum: T = Default::default();
                for k in 0..self.cols {
                    sum += *self.get(i, k).unwrap() * *b_t.get(j, k).unwrap();
                }

                new_values.push(sum);
            }
        }

        Ok(Matrix::new(matrix_b.rows, self.cols, new_values))
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
            Err(e) => panic!("{}", e),
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
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn check_transpose() {
        let matrix_a: Matrix<i32> = Matrix {
            rows: 2,
            cols: 3,
            values: vec![1, -3, 5, -9, 4, 7],
        };

        let expected_result: Matrix<i32> = Matrix {
            rows: 3,
            cols: 2,
            values: vec![1, -9, -3, 4, 5, 7],
        };

        let a_t = matrix_a.transpose();
        assert_eq!(a_t, expected_result);
    }

    #[test]
    fn check_naive() {
        let matrix_a: Matrix<i32> = Matrix {
            rows: 2,
            cols: 3,
            values: vec![1, 2, 3, 4, 5, 6],
        };

        let mut matrix_b: Matrix<i32> = Matrix {
            rows: 3,
            cols: 2,
            values: vec![7, 8, 9, 10, 11, 12],
        };

        let expected_result: Matrix<i32> = Matrix {
            rows: 2,
            cols: 2,
            values: vec![58, 64, 139, 154],
        };

        match matrix_a.mult_naive(&mut matrix_b) {
            Ok(result) => assert_eq!(result.values, expected_result.values),
            Err(e) => panic!("{}", e),
        }
    }
}
