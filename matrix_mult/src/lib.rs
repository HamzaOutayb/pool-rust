use lalgebra_scalar::Scalar;
use std::{clone, ops::Mul};
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        let mut col: usize = 0;
        for i in 0..self.0.len() {
            col = col + 1;
        }
        col
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut v = vec![];

        for num in self.0.iter() {
            v.push(num[n].clone());
        }

        v
    }
}

impl<T: Scalar<Item = T> + Mul<Output = T> + Copy + std::ops::Add<Output = T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn mul(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = Vec::new();

        result.push(vec![
            self.0[0][0] * other.0[1][0] + self.0[0][1] * other.0[1][0],
            self.0[0][0] * other.0[1][1] + self.0[0][1] * other.0[1][1],
        ]);

        result.push(vec![
            self.0[1][0] * other.0[1][0] + self.0[1][1] * other.0[1][0],
            self.0[1][0] * other.0[1][1] + self.0[1][1] * other.0[1][1],
        ]);

        Some(Matrix(result))
    }
}
