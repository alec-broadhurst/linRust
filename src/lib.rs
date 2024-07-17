pub enum DType {
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Float(f32),
    Double(f64),
}

pub struct Matrix {
    rows: usize,
    cols: usize,
    dtype: DType,
    values: Vec<DType>,
}

impl Matrix {
    fn new(rows: usize, cols: usize, dtype: DType, values: Vec<DType>) -> Self {
        Matrix {
            rows,
            cols,
            dtype,
            values,
        }
    }
}
