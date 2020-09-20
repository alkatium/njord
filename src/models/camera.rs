use crate::geometry::point::Point;
use crate::geometry::vector::Vector;

use crate::components::image::Image;
use crate::models::scene::Scene;

#[derive(Debug)]
pub struct Camera {
    pub position: Point,
    pub target: Vector,
    pub distance: Vector
}

impl Camera {
    pub fn genererImage(scene: Scene, image: Image){
        println!("Here we generate the image")
    }
}