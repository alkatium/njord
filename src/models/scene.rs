use crate::geometry::ray::Ray;
use crate::geometry::intersection::Intersection;

use crate::components::color::Color;

use crate::shapes::object::Object;

use crate::models::light::Light;

pub struct Scene {
    
    pub background: Color,
    pub ambiant: Light,
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>
}

impl Scene {
    pub fn addObject(mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn addLight(mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn init(&self, filename: String) {

    }

    pub fn intersect(&self, ray: &Ray, inter: &mut Intersection) -> bool {
        let mut checked: bool = false;

        for obj in &self.objects {

            let mut intersection =  Intersection::new();

            if obj.intersect(&ray, &mut intersection) {
                if intersection.distance < inter.distance {
                    *inter = intersection;
                }

                checked = true;
            }
        }

        return checked;
    }

    pub fn cut(&self, ray: &Ray) -> bool {

        for obj in &self.objects {
            if obj.cut(ray) {
                return true;
            }
        }

        return false;
    }
}