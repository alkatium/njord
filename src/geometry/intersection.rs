use crate::shapes::object::Object;

use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::geometry::ray::Ray;

use crate::models::scene::Scene;

use crate::components::color::Color;

pub struct Intersection {

    pub p: Point,
    pub object: Option<Box<dyn Object>>,
    pub distance: f32,
}

impl Intersection {
    pub fn new() -> Intersection {

        // by default none object instance
        return Intersection {
            p: Point::center(),
            object: None,
            distance: 3.3_f32
        }
    }

    fn getCouleur(mut self, obs: &mut Intersection, scene: &Scene, depth: &mut u32) -> Color {

        let mut reflected = Color::black();

        let ambiant_intensity = match &self.object {
            // dynamic object exists
            Some(o) => o.ambiant(&scene.ambiant.intensity),
            // object is None, hence default Color
            None    => Color::black()
        };

        let direct = match &self.object {
            // dynamic object exists
            Some(o) => o.direct(&obs.p, &scene, &self),
            // object is None, hence default Color
            None    => Color::black()
        };

        let specular = match &self.object {
            // dynamic object exists
            Some(o) => o.isSpecular(),
            // object is None, hence default false
            None    => false
        };

        if *depth > 0 && specular {

            let r: Ray = match &self.object {
                // dynamic object exists
                Some(o) => o.getReflected(&obs.p, &self),
                // object is None, hence default false
                None    => Ray::default()
            };

            let mut inter: Intersection = Intersection::new();

            if scene.intersect(&r, &mut inter) {
                *depth = *depth - 1;
                reflected = inter.getCouleur(&mut self, scene, depth)
            }

        }

        // compute final color value
        let sum: Color = ambiant_intensity + direct + reflected;

        return sum;
    }

    pub fn getNormal(&self) -> Vector {

        return match &self.object {
            // dynamic object exists
            Some(o) => o.getNormal(&self.p),
            // object is None, hence default false
            None    => Vector::default()
        };
    }
}