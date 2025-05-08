#[derive(PartialEq, Eq)]
#[derive(Debug)]
struct Matrix((i32, i32)(i32, i32))

pub fn transpose(m: Matrix) -> Matrix {
    let matrix: Matrix = Matrix((m.1.0, m.1.1), (m.0.0, m.0.1))
    matrix
}