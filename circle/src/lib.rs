#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
    pub fn new(point1: f64, point2: f64, radius: f64) -> Circle {
        Circle {
            center: Point(point1, point2),
            radius
        }
    }
    //-> returns the diameter of the circle.
    pub fn diameter(self) -> f64 {
        2.0*self.radius
    } 

    //-> returns the area of the circle.
    pub fn area(self) -> f64 {
        3.14159*self.diameter()
    } 

    //-> returns if two circles intersect.
    pub fn intersect(self, other: Self) -> bool {
        self.diameter()+other.diameter() == self.center.distance(other.center)
    } 

}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    //-> returns the distance between two coordinates.
    pub fn distance(self, other: Self) -> f64 {
        ((self.0 - other.0).powf(2.0) + (self.1 - other.1).powf(2.0)).sqrt()
    }
}