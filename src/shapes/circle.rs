use crate::geometry::point::Point;
use crate::geometry::vector::vector;
use crate::geometry::intersection::Intersection;

use crate::shapes::object::Object;

use crate::components::material::Material;

pub struct Circle {
    pub mat: Material;
    pub center: Point,
    pub radius: f32
}

impl Object for Circle {

    fn new() -> Circle {
        mat: Material {
            color: Color::one(),
            kd: 10.,
            ks: 10.,
            s: 10.
        }, 
        center: Point {x: 0., y:0.}, 
        radius: 1.
    }

    fn intersect(&self, r: Ray, inter: Intersection) -> bool {
        let dist_origin_center = self.centre.dist(self.r.getOrigin());

        return false;
    }

    fn getNormal(&self, p: Point) -> Vector {
        return Vector {
            dx: p.x - self.centre.x,
            dx: p.y - self.centre.y,
            dx: p.z - self.centre.z
        }
    }
}