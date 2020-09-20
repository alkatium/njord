use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::geometry::ray::Ray;
use crate::geometry::intersection::Intersection;

use crate::components::color::Color;
use crate::components::image::Image;
use crate::models::scene::Scene;

const IMAGE_SIZE:u32 = 2;

#[derive(Debug)]
pub struct Camera {
    pub position: Point,
    pub target: Vector,
    pub distance: Vector
}

impl Camera {

    pub fn default() -> Camera {
        Camera {
            position: Point {
                x: 0.,
                y: 0.,
                z: 2.
            },
            target: Vector {
                dx: 0.,
                dy: 0.,
                dz: -1.
            },
            distance: Vector {
                dx: 1.,
                dy: 1.,
                dz: -1.
            }
        }
    }

    pub fn generer_image(&self, scene: &mut Scene, image: &mut Image){

        for i in 0..image.height {
            for j in 0..image.width {
    
                if i == j {
                    image.set_pixel(i, j, &Color::white());
                }else{
                    image.set_pixel(i, j, &scene.background);
                }
            }
        }
    
        // ajouter un lancer de rayon pour chaque pixel
        let middle_plane: f32 = IMAGE_SIZE as f32 / 2.; 
    
        let width_pixel: f32 = IMAGE_SIZE as f32 / image.width as f32;
        let height_pixel: f32 = IMAGE_SIZE as f32 / image.height as f32;
    
        let mut middle_x: f32;
        let mut middle_y : f32;
    
        for i in 0..image.height {
            for j in 0..image.width {

                middle_x = width_pixel * i as f32 - middle_plane + width_pixel / 2.;
                middle_y = -(height_pixel * j as f32 - middle_plane + width_pixel / 2.);
    
                let dir = Vector {
                    dx: middle_x * self.distance.dx, 
                    dy: middle_y * self.distance.dy, 
                    dz: 1. * self.distance.dz
                };

                let r: Ray = Ray {
                    origin: self.position,
                    direction: dir
                };
    
                let mut inter: Intersection = Intersection::new();

                if scene.intersect(&r, &mut inter)
                {
                    // &inter.get_couleur(&inter, scene, &mut 2)
                    image.set_pixel(i, j, &inter.get_couleur(&inter, scene, &mut 2));
                }
                else
                {
                    // println!("No intersection: {:?}", &scene.background);
                    image.set_pixel(i, j, &scene.background);
                }
            }
        }
    }
}