#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self, other: Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            center: Point(x, y),
            radius,
        }
    }

    pub fn diameter(self) -> f64 {
        2.0 * self.radius
    }

    pub fn area(self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn intersect(self, other: Circle) -> bool {
        let distance_between_centers = self.center.distance(other.center);
        distance_between_centers < (self.radius + other.radius)
    }
}
