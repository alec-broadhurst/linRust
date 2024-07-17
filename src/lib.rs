pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    values: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            let index = (row * self.cols) + col;
            Some(&self.values[index])
        } else {
            None
        }
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
}
