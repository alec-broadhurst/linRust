pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    values: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            let index = (row * self.cols) + row;
            Some(&self.values[index])
        } else {
            None
        }
    }
}
