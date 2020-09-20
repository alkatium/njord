use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::geometry::ray::Ray;
use crate::geometry::point::Point;
use crate::geometry::intersection::Intersection;

use crate::components::color::Color;
use crate::components::material::Material;

use crate::shapes::object::Object;
use crate::shapes::sphere::Sphere;

use crate::models::light::Light;
use crate::models::light::Intensity;

pub struct Scene {
    
    pub background: Color,
    pub ambient: Light,
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>
}

impl Scene {

    pub fn init(filename: &'static str) -> Scene {

        // read scene file and extract each element
        // let plane_str = "plane";
        let background_str = "background";
        let material_str = "material";
        let sphere_str = "sphere";
        let light_str = "light";
        let ambient_str = "ambient";

        let file = File::open(filename).expect("Read file failed");
        let reader = BufReader::new(file);

        // usefull data
        let mut ambient = Light::default();
        let mut material: Material = Material::default();
        let mut background = Color::white();
        let mut objects_list: Vec<Box<dyn Object>> = Vec::new();
        let mut lights_list: Vec<Light> = Vec::new();

        for line in reader.lines() {

            let current_line = String::from(line.expect("error while reading line").trim().replace("\n", ""));
            let splited: Vec<&str> = current_line.split(" ").collect();

            // Extract material
            if current_line.starts_with(material_str) {

                let r: f32 = String::from(splited[1]).parse().expect("Unknown red color for material");
                let g: f32 = String::from(splited[2]).parse().expect("Unknown green color for material");
                let b: f32 = String::from(splited[3]).parse().expect("Unknown blue color for material");

                let color = Color {
                    r: r,
                    g: g,
                    b: b
                };

                let ka: f32 = String::from(splited[4]).parse().expect("Unknown ka for material");
                let kd: f32 = String::from(splited[4]).parse().expect("Unknown kd for material");
                let ks: f32 = String::from(splited[6]).parse().expect("Unknown ks for material");
                let s: f32 = String::from(splited[7]).parse().expect("Unknown s for material");

                material = Material {
                    color: color,
                    ka: ka,
                    kd: kd,
                    ks: ks,
                    s: s
                };
            }

            // extract background
            if current_line.starts_with(background_str) {

                let r: f32 = String::from(splited[1]).parse().expect("Unknown red color for background");
                let g: f32 = String::from(splited[2]).parse().expect("Unknown green color for background");
                let b: f32 = String::from(splited[3]).parse().expect("Unknown blue color for background");

                background = Color {
                    r: r,
                    g: g,
                    b: b
                };
            }

            // extract sphere
            if current_line.starts_with(sphere_str) {

                let x: f32 = String::from(splited[1]).parse().expect("Unknown x value for sphere coordinate");
                let y: f32 = String::from(splited[2]).parse().expect("Unknown y value for sphere coordinate");
                let z: f32 = String::from(splited[3]).parse().expect("Unknown z value for sphere coordinate");

                let radius: f32 = String::from(splited[4]).parse().expect("Unknown radius value for sphere");

                let sphere = Sphere {
                    mat: material,
                    center: Point {
                        x: x, 
                        y: y,
                        z: z
                    },
                    radius: radius
                };
                
                objects_list.push(Box::new(sphere));
            }

            // extract light
            if current_line.starts_with(light_str) {

                let x: f32 = String::from(splited[1]).parse().expect("Unknown x value for light coordinate");
                let y: f32 = String::from(splited[2]).parse().expect("Unknown y value for light coordinate");
                let z: f32 = String::from(splited[3]).parse().expect("Unknown z value for light coordinate");

                let er: f32 = String::from(splited[4]).parse().expect("Unknown red energy for light");
                let eg: f32 = String::from(splited[5]).parse().expect("Unknown green energy for light");
                let eb: f32 = String::from(splited[6]).parse().expect("Unknown blue energy for light");

                let light = Light {
                    position: Point {
                        x: x,
                        y: y,
                        z: z
                    },
                    intensity: Intensity {
                        er: er,
                        eg: eg,
                        eb: eb
                    }
                };

                lights_list.push(light);
            }

            // extract ambient light
            if current_line.starts_with(ambient_str) {

                let er: f32 = String::from(splited[1]).parse().expect("Unknown red energy for ambient");
                let eg: f32 = String::from(splited[2]).parse().expect("Unknown green energy for ambient");
                let eb: f32 = String::from(splited[3]).parse().expect("Unknown blue energy for ambient");

                ambient.intensity = Intensity {
                    er: er,
                    eg: eg,
                    eb: eb
                }
            }
        }

        return Scene {
            background: background,
            ambient: ambient,
            objects: objects_list,
            lights: lights_list
        };
    }

    // pub fn addObject(mut self, object: Box<dyn Object>) {
    //     self.objects.push(object);
    // }

    // pub fn addLight(mut self, light: Light) {
    //     self.lights.push(light);
    // }

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