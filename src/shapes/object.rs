use crate::geometry::ray::Ray;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::geometry::intersection::Intersection;

use crate::components::material::Material;
use crate::components::color::Color;

use crate::model::scene::Scene;

pub trait Object {

    fn intersect(&self, r: Ray, inter: Intersection) -> bool;
    fn getNormal(&self, p: Point) -> Vector;
    fn getMaterial(&self) -> Material;
    fn direct(&self, obs: Point, sc: Scene, inter: Intersection) -> Color;
}