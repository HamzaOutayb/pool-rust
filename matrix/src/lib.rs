use lalgebra_scalar::Scalar;
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T> + std::clone::Clone> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

	pub fn identity(n: usize) -> Matrix<T> {
        let mut v = Vec::new();
        for i in 0..n {
            v.push(vec![T::zero();n]);
            v[i][i] = T::one();
        }
        Matrix(v)
	}
}