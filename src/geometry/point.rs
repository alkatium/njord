use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point {
    pub fn dist(&self, p: &Point) -> f32 {
        return ((p.x - self.x) * (p.x - self.x) +
                (p.y - self.y) * (p.y - self.y) + 
                (p.z - self.z) * (p.z - self.z))
                .sqrt();
    }

    pub fn center() -> Point {
        Point {
            x: 0.,
            y: 0.,
            z: 0.
        }
    }
}

impl ops::Sub<Point> for Point {

    type Output = Point;

    fn sub(self, p: Point) -> Point {
        return Point { 
            x: self.x - p.x, 
            y: self.y - p.y, 
            z: self.z - p.z
        };
    }
}

impl ops::Add<Point> for Point {

    type Output = Point;

    fn add(self, p: Point) -> Point {
        return Point { 
            x: self.x + p.x, 
            y: self.y + p.y, 
            z: self.z + p.z
        };
    }
}