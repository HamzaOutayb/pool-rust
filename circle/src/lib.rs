use std::f64::consts::PI;

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
        self.radius*2.0
    } 

    //-> returns the area of the circle.
    pub fn area(self) -> f64 {
        PI*self.radius*self.radius
    } 

    //-> returns if two circles intersect.
        pub fn intersect(self, circle2: Circle) -> bool {
            let rigth = self.radius + circle2.radius;
            let d = self.center.distance(circle2.center);
            if rigth >= d  {
                return true;
            }
            false
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

