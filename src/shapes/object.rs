use crate::geometry::ray::Ray;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::geometry::intersection::Intersection;

use crate::components::material::Material;
use crate::components::color::Color;

use crate::models::scene::Scene;
use crate::models::light::Intensity;

pub trait Object {

    fn new() -> Self where Self: Sized;

    fn getNormal(&self, p: &Point) -> Vector;
    fn getMaterial(&self) -> Material;
    fn isSpecular(&self) -> bool;

    fn direct(&self, obs: &Point, sc: &Scene, inter: &Intersection) -> Color;
    fn ambiant(&self, ambiant: &Intensity) -> Color;

    fn intersect(&self, r: &Ray, inter: &mut Intersection) -> bool;
    fn cut(&self, ray: &Ray) -> bool;
    fn getReflected(&self, obs: &Point, inter: &Intersection) -> Ray;
}