#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::{Add, Mul};
use lalgebra_scalar::Scalar;

impl<T: Scalar + Add<Output = T> + Copy> Add for Vector<T> {
    type Output = Option<Vector<T>>;
    fn add(self, rhs: Self)-> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None
        }
        let mut res = Vec::new();
        for index in 0..self.0.len() {
            res.push(self.0[index] + rhs.0[index]);
        }
        Some(Vector(res))
    }
}

impl<T: Scalar<Item = T> + Mul<Output = T> + Add<Output = T> + Copy> Vector<T> {
	pub fn new() -> Self {
        Vector(Vec::new())
	}

	pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None
        }
        let mut res = T::zero();
        for index in 0..self.0.len() {
            res = res + (self.0[index] * other.0[index])
        }
        Some(res)
	}
}