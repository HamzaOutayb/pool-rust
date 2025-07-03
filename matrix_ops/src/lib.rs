//use matrix::Matrix;
use std::ops::Add;
use std::ops::Sub;
use lalgebra_scalar::Scalar;
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Add<Output = T> + Copy> Add for Matrix<T>
{
        type Output = Option<Matrix<T>>;
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut vec = vec![];
        for i in 0..self.0.len() {
            if self.0[i].len() != other.0[i].len() {
                return None
            }
            let mut s = Vec::new();
            for j in 0..other.0[i].len() {
                let mut sum = T::zero();
                sum = self.0[i][j] + other.0[i][j];
                s.push(sum);
            }
            vec.push(s);
        }
        Some(Matrix(vec))
    }
}

impl<T: Scalar<Item = T> + Sub<Output = T> + Copy> Sub for Matrix<T> 
{
       type Output = Option<Matrix<T>>;
    fn sub(self, other: Self) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut vec = vec![];
        for i in 0..self.0.len() {
            if self.0[i].len() != other.0[i].len() {
                return None
            }
            let mut s = Vec::new();
            for j in 0..other.0[i].len() {
                let mut sum = T::zero();
                sum = self.0[i][j] - other.0[i][j];
                s.push(sum);
            }
            vec.push(s);
        }
        Some(Matrix(vec))
    }
}