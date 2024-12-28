use crate::error::MatrixError;
use crate::identity_element::IdentityElement;
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    values: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Copy
        + Default
        + AddAssign
        + IdentityElement,
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

    pub fn set_rows(&mut self, new_rows: usize) -> &mut Self {
        self.rows = new_rows;
        self
    }

    pub fn set_cols(&mut self, new_cols: usize) -> &mut Self {
        self.cols = new_cols;
        self
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
                "Cannot add matrices of dimensions {}x{} and {}x{}",
                self.rows, self.cols, matrix_b.rows, matrix_b.cols
            )));
        }

        let new_values: Vec<T> = self
            .values
            .iter()
            .zip(&matrix_b.values)
            .map(|(a, b)| *a + *b)
            .collect();

        Ok(Matrix::new(self.rows, self.cols, new_values))
    }

    pub fn add_mut(&mut self, matrix_b: &Matrix<T>) -> Result<&mut Self, MatrixError> {
        if self.rows != matrix_b.rows || self.cols != matrix_b.cols {
            return Err(MatrixError::DimensionMismatch(format!(
                "Cannot add matricies of dimensions {}x{} and {}x{}",
                self.rows, self.cols, matrix_b.rows, matrix_b.cols
            )));
        }

        for i in 0..self.values.len() {
            self.values[i] = self.values[i] + matrix_b.values[i];
        }

        Ok(self)
    }

    pub fn subtract(&mut self, matrix_b: &Matrix<T>) -> Result<&mut Self, MatrixError> {
        if self.rows != matrix_b.rows || self.cols != matrix_b.cols {
            return Err(MatrixError::DimensionMismatch(format!(
                "Cannot subtract matricies of dimensions {}x{} and {}x{}",
                self.rows, self.cols, matrix_b.rows, matrix_b.cols
            )));
        }

        for i in 0..self.values.len() {
            self.values[i] = self.values[i] - matrix_b.values[i];
        }

        Ok(self)
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut new_values: Vec<T> = vec![T::default(); self.rows * self.cols];

        for i in 0..self.cols {
            for j in 0..self.rows {
                new_values[i * self.rows + j] = self.values[j * self.cols + i];
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

        let bt = matrix_b.transpose();

        let mut new_values: Vec<T> = vec![T::default(); self.rows * matrix_b.cols];

        for i in 0..self.rows {
            for j in 0..bt.rows {
                let mut sum: T = Default::default();
                for k in 0..self.cols {
                    sum += self.values[i * self.cols + k] * bt.values[j * bt.cols + k];
                }

                new_values[i * matrix_b.cols + j] = sum;
            }
        }

        Ok(Matrix::new(self.rows, matrix_b.cols, new_values))
    }

    pub fn mult_scalar(&mut self, num: T) -> &mut Self {
        for value in &mut self.values {
            *value = *value * num;
        }

        self
    }

    pub fn identity(order: usize) -> Matrix<T> {
        let mut values: Vec<T> = vec![T::zero(); order * order];

        for i in 0..order {
            values[i * order + i] = T::one();
        }

        Matrix::new(order, order, values)
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
        let mut matrix_a: Matrix<u32> = Matrix::new(2, 2, vec![1, 2, 3, 4]);
        let matrix_b: Matrix<u32> = Matrix::new(2, 2, vec![5, 6, 7, 8]);
        let expected_result: Vec<u32> = vec![6, 8, 10, 12];

        match matrix_a.add_mut(&matrix_b) {
            Ok(_) => assert_eq!(matrix_a.values, expected_result),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn check_subtraction() {
        let mut matrix_a: Matrix<i32> = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let matrix_b: Matrix<i32> = Matrix::new(3, 3, vec![10, 21, 12, 13, 14, 15, 16, 17, 18]);
        let expected_result: Vec<i32> = vec![-9, -19, -9, -9, -9, -9, -9, -9, -9];

        match matrix_a.subtract(&matrix_b) {
            Ok(_) => assert_eq!(matrix_a.values, expected_result),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn check_transpose() {
        let matrix_a: Matrix<i32> = Matrix::new(2, 3, vec![1, -3, 5, -9, 4, 7]);
        let expected_result: Matrix<i32> = Matrix::new(3, 2, vec![1, -9, -3, 4, 5, 7]);

        let matrix_at = matrix_a.transpose();
        assert_eq!(matrix_at, expected_result);
    }

    #[test]
    fn check_naive() {
        let matrix_a: Matrix<i32> = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let mut matrix_b: Matrix<i32> = Matrix::new(3, 2, vec![7, 8, 9, 10, 11, 12]);
        let expected_result: Vec<i32> = vec![58, 64, 139, 154];

        match matrix_a.mult_naive(&mut matrix_b) {
            Ok(matrix_c) => assert_eq!(matrix_c.values, expected_result),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn check_scalar_multiplication() {
        let mut matrix: Matrix<i32> = Matrix::new(2, 2, vec![1, -2, 3, 4]);
        let scalar = 3;
        let expected_result: Matrix<i32> = Matrix::new(2, 2, vec![3, -6, 9, 12]);

        matrix.mult_scalar(scalar);

        assert_eq!(matrix, expected_result);
    }

    #[test]
    fn check_identity() {
        let identity_matrix_2x2: Matrix<f64> = Matrix::identity(2);
        let expected_result_2x2: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]);

        let identity_matrix_10x10: Matrix<u16> = Matrix::identity(10);
        let values: Vec<u16> = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let expected_result_10x10: Matrix<u16> = Matrix::new(10, 10, values);

        assert_eq!(identity_matrix_2x2, expected_result_2x2);
        assert_eq!(identity_matrix_10x10, expected_result_10x10)
    }
}
