use std::fs::File;
use std::io::prelude::*;

use crate::components::color::Color;

pub struct Image {
    pub pixels: Vec<Color>,
    pub width: u32,
    pub height: u32
}

impl Image {

    pub fn new(width: u32, height: u32) -> Image {

        // initialize vector of color
        let size = width * height;
        let mut color_vec: Vec<Color> = Vec::with_capacity(size as usize);

        // set color as black
        for _ in 0..size {
            color_vec.push(Color::black());
        }
        
        Image {
            width: width,
            height: height,
            pixels: color_vec
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, c: &Color) {
        // check if possible to set new color
        if x <= (self.width - 1) {
            if y <= (self.height - 1) {
                self.pixels[(y * self.width + x) as usize] = *c;
                return;
            }
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        // find if exists
        if x <= (self.width - 1) {
            if y <= (self.height - 1) {
                return self.pixels[(y * self.width + x) as usize];
            }
        }

        // by default..
        return Color::black();
    }

    #[warn(unused_must_use)]
    pub fn save(&self, filename: &'static str) {

        let mut file = File::create(filename).expect("File creation failed");
        file.write("P3\n".as_bytes()).expect("Error while writing");
        file.write(format!("{} {}\n", self.width, self.height).as_bytes()).expect("Error while writing");
        file.write("255\n".as_bytes()).expect("Error while writing");

        for y in 0..self.height {
            for x in 0..self.width {

                // TODO : check if correct
                let index: usize = (y * self.width + x) as usize;
                let color: Color = self.pixels[index];
                file.write(format!("{} {} {}  ", (color.r * 255.) as u32, (color.g * 255.) as u32, (color.b * 255.) as u32).as_bytes())
                    .expect("Error while writing");
            }

            file.write("\n".as_bytes()).expect("Error while writing");
        }
    }
}