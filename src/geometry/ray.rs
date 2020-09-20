use crate::geometry::point::Point;
use crate::geometry::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Ray {

    pub fn default() -> Ray {
        Ray {
            origin: Point::center(),
            direction: Vector::default()
        }
    }
}