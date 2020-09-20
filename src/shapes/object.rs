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
    fn intersect(&self, r: &Ray, inter: &mut Intersection) -> bool;
    fn cut(&self, ray: &Ray) -> bool;

    fn direct(&self, obs: &Point, scene: &Scene, inter: &Intersection) -> Color {

        let mut total = Color::black();

        // diffus part
        for s in &scene.lights {

            let l_normalized: Vector = Vector {
                dx: s.position.x - inter.p.x,
                dy: s.position.y - inter.p.y,
                dz: s.position.z - inter.p.z,
            }.normalized();

            // check if ray encounters object and generate shade
            let ray: Ray = Ray {
                origin: inter.p,
                direction: l_normalized
            };

            if !scene.cut(&ray) {

                let kd: Color = self.getMaterial().color * self.getMaterial().kd;

                let n: Vector  = inter.getNormal().normalized();
    
                let d_scalar: f32 = l_normalized * n;

                if d_scalar > 0. {

                    let id: Color = kd * d_scalar * &s.intensity;

                    total += id;
                }

                // specular part
                let v = Vector {
                    dx: obs.x - inter.p.x,
                    dy: obs.y - inter.p.y,
                    dz: obs.z - inter.p.z,   
                }.normalized();

                let r: Vector = n * 2. * (n * l_normalized) - l_normalized;

                let s_scalar: f32 = v * r;

                if s_scalar > 0. {
                  let col_ks: Color = Color{
                      r: self.getMaterial().ks, 
                      g: self.getMaterial().ks, 
                      b: self.getMaterial().ks
                  };

                  let Is: Color = col_ks * s_scalar.powf(self.getMaterial().s) * s.intensity;
          
                  total += Is;
                }
            }
        }

        return total;
    }

    fn ambiant(&self, ambiant: &Intensity) -> Color {
        return self.getMaterial().color * self.getMaterial().ka * ambiant;
    }

    fn getReflected(&self, obs: &Point, inter: &Intersection) -> Ray {

        let l_normalized = Vector {
            dx: obs.x - inter.p.x, 
            dy: obs.y - inter.p.y, 
            dz: obs.z - inter.p.z
        }.normalized();

        let n = inter.getNormal().normalized();

        let r = (n * 2. * (n * l_normalized) - l_normalized).normalized();

        return Ray {
            origin: inter.p,
            direction: r
        }
    }
}