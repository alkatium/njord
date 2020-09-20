use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::geometry::ray::Ray;
use crate::geometry::intersection::Intersection;

use crate::shapes::object::Object;

use crate::components::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub mat: Material,
    pub center: Point,
    pub radius: f32
}

impl Object for Sphere {

    fn new() -> Sphere {
        Sphere {
            mat: Material::default(), 
            center: Point::center(), 
            radius: 2.
        }
    }

    fn get_material(&self) -> Material {
        return self.mat;
    }

    fn is_specular(&self) -> bool { 

        println!("Ks {}", self.mat.ks);

        if self.mat.ks == 0. {
            return false;
        }
        else {
            return true;
        }
    }

    fn intersect(&self, r: &Ray, inter: &mut Intersection) -> bool {
        
        // keep in memory values
        let dist_origin_center = r.origin.dist(&self.center);
        let ray_direction_norm = r.direction.norm();

        // compute equation values
        let eq_a = ray_direction_norm * ray_direction_norm;

        // need to compute separatly
        let eq_b = 2. * (r.direction.dx * (r.origin.x - self.center.x) 
                    + r.direction.dy * (r.origin.y - self.center.y) 
                    + r.direction.dz * (r.origin.z - self.center.z)); 
        
        let eq_c = (dist_origin_center * dist_origin_center) - (self.radius * self.radius);
        
        // compute delta of polynomial equation
        let delta = (eq_b * eq_b) - (4. * eq_a * eq_c);

        let mut t: f32 = 3.3_f32;

        // check intersect from delta value
        if delta == 0.{
            
            t = -eq_b / (2. * eq_a);

        } if delta > 0. {

            let t1 = (-eq_b + delta.sqrt()) / (2. * eq_a);
            let t2 = (-eq_b - delta.sqrt()) / (2. * eq_a);

            t = t1.min(t2);
            
            if t < f32::EPSILON {
                t = t1.max(t2);
            }
        }

        // update value if t value is better
        if !(delta < 0.) {

            let x = r.origin.x + r.direction.dx_norm() * t;
            let y = r.origin.y + r.direction.dy_norm() * t;
            let z = r.origin.z + r.direction.dz_norm() * t;

            inter.distance = t;
            inter.object = Some(Box::new(*self));

            inter.p.x = x;
            inter.p.y = y;
            inter.p.z = z;

            return true;
        }
        
        return false;
    }

    fn get_normal(&self, p: &Point) -> Vector {
        return Vector {
            dx: p.x - self.center.x,
            dy: p.y - self.center.y,
            dz: p.z - self.center.z
        }
    }
    fn cut(&self, r: &Ray) -> bool { 

        // keep in memory values
        let dist_origin_center = r.origin.dist(&self.center);
        let ray_direction_norm = r.direction.norm();

        // compute equation values
        let eq_a = ray_direction_norm * ray_direction_norm;

        // need to compute separatly
        let eq_b = 2. * (r.direction.dx * (r.origin.x - self.center.x) 
                    + r.direction.dy * (r.origin.y - self.center.y) 
                    + r.direction.dz * (r.origin.z - self.center.z)); 
        
        let eq_c = (dist_origin_center * dist_origin_center) - (self.radius * self.radius);
        
        // compute delta of polynomial equation
        let delta: f32 = (eq_b * eq_b) - (4. * eq_a * eq_c);

        let mut t: f32 = 3.3_f32;

        if delta < 0. {
            return false;
        }

        // check intersect from delta value
        if delta == 0.{
            
            t = -eq_b / (2. * eq_a);

        } if delta > 0. {

            let t1 = (-eq_b + delta.sqrt()) / (2. * eq_a);
            let t2 = (-eq_b - delta.sqrt()) / (2. * eq_a);

            t = t1.min(t2);
            
            if t < f32::EPSILON {
                t = t1.max(t2);
            }
        }

        if t - f32::EPSILON > 0. && t + f32::EPSILON < 1. {
            return true;
        }
        
        return false;
    }
}