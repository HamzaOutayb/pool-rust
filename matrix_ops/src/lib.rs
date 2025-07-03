use matrix::Matrix;

impl Add for Matrix {
    fn add(&self, other: self) -> Option<Matrix(T)> {
        if self.0.len() != other.self.0.len() {
            return None
        }
    }
}

impl Sub for Matrix {

}