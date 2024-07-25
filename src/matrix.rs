use crate::error::MatrixError;
use std::ops::{Add, AddAssign, Mul, Sub};

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

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_values(&self) -> &Vec<T> {
        &self.values
    }

    pub fn set_rows(&mut self, new_rows: usize) {
        self.rows = new_rows;
    }

    pub fn set_cols(&mut self, new_cols: usize) {
        self.cols = new_cols;
    }

    pub fn set_values(&mut self, new_values: Vec<T>) -> Result<(), MatrixError> {
        if new_values.len() == self.rows * self.cols {
            self.values = new_values;
            Ok(())
        } else {
            Err(MatrixError::DimensionMismatch(format!(
                "Matrix has capacity of {}, gave it {} values",
                self.rows * self.cols,
                new_values.len()
            )))
        }
    }

    pub fn value_at(&self, row: usize, col: usize) -> Result<&T, MatrixError> {
        if row < self.rows && col < self.cols {
            let index = (row * self.cols) + col;
            Ok(&self.values[index])
        } else {
            Err(MatrixError::InvalidIndex(format!(
                "Index ({}, {}) is out of bounds for matrix of size {}x{}",
                row, col, self.rows, self.cols
            )))
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
                new_values.push(*self.value_at(j, i).unwrap());
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
                    sum += *self.value_at(i, k).unwrap() * *b_t.value_at(j, k).unwrap();
                }

                new_values.push(sum);
            }
        }

        Ok(Matrix::new(matrix_b.rows, self.cols, new_values))
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn check_indexing() {
        let matrix: Matrix<u32> = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(matrix.value_at(0, 0).unwrap(), &1);
        assert_eq!(matrix.value_at(1, 1).unwrap(), &5);
        assert_eq!(matrix.value_at(2, 0).unwrap(), &7);

        assert!(matrix.value_at(3, 0).is_err());
        assert!(matrix.value_at(0, 3).is_err());
        assert!(matrix.value_at(3, 3).is_err());
    }

    #[test]
    fn check_addition() {
        let matrix_a: Matrix<u32> = Matrix::new(2, 2, vec![1, 2, 3, 4]);
        let matrix_b: Matrix<u32> = Matrix::new(2, 2, vec![5, 6, 7, 8]);
        let expected_result: Matrix<u32> = Matrix::new(2, 2, vec![6, 8, 10, 12]);

        match matrix_a.add(&matrix_b) {
            Ok(result) => assert_eq!(result, expected_result),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn check_subtraction() {
        let matrix_a: Matrix<i32> = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let matrix_b: Matrix<i32> = Matrix::new(3, 3, vec![10, 21, 12, 13, 14, 15, 16, 17, 18]);
        let expected_result: Matrix<i32> =
            Matrix::new(3, 3, vec![-9, -19, -9, -9, -9, -9, -9, -9, -9]);

        match matrix_a.subtract(&matrix_b) {
            Ok(result) => assert_eq!(result, expected_result),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn check_transpose() {
        let matrix_a: Matrix<i32> = Matrix::new(2, 3, vec![1, -3, 5, -9, 4, 7]);
        let expected_result: Matrix<i32> = Matrix::new(3, 2, vec![1, -9, -3, 4, 5, 7]);

        let a_t = matrix_a.transpose();
        assert_eq!(a_t, expected_result);
    }

    #[test]
    fn check_naive() {
        let matrix_a: Matrix<i32> = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let mut matrix_b: Matrix<i32> = Matrix::new(3, 2, vec![7, 8, 9, 10, 11, 12]);
        let expected_result: Matrix<i32> = Matrix::new(2, 2, vec![58, 64, 139, 154]);

        match matrix_a.mult_naive(&mut matrix_b) {
            Ok(result) => assert_eq!(result.values, expected_result.values),
            Err(e) => panic!("{}", e),
        }
    }
}
