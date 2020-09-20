use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::geometry::ray::Ray;
use crate::geometry::point::Point;
use crate::geometry::intersection::Intersection;

use crate::components::color::Color;
use crate::components::material::Material;

use crate::shapes::object::Object;

use crate::models::light::Light;
use crate::models::light::Intensity;

pub struct Scene {
    
    pub background: Color,
    pub ambiant: Light,
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>
}

impl Scene {

    pub fn init(filename: &'static str) -> Scene {


        let planeStr = "plan";
        let backgroundStr = "fond";
        let materialStr = "materiau";
        let sphereStr = "sphere";
        let lightStr = "source";
        let ambiantStr = "ambiant";

        let file = File::open(filename).expect("Read file failed");
        let reader = BufReader::new(file);

        let mut material: Material;

        for line in reader.lines() {
            let currentLine = String::from(line.expect("error while reading line").trim());

            if currentLine.contains("material") {
                println!("Material found");
            }
        }
        
        let background = Color::white();
        let mut objects_list: Vec<Box<dyn Object>> = Vec::new();
        let mut lights_list: Vec<Light> = Vec::new();

        return Scene {
            background: background,
            ambiant: Light {
                position: Point::center(),
                intensity: Intensity::default()
            },
            objects: objects_list,
            lights: lights_list
        }

    }

    pub fn addObject(mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn addLight(mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn intersect(&mut self, ray: &Ray, inter: &mut Intersection) -> bool {
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
            if obj.cut(&ray) {
                return true;
            }
        }

        return false;
    }
}