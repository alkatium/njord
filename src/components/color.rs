use std::ops;

use crate::models::light::Intensity;

#[derive(Debug)]
pub struct Color {
    r: u32,
    g: u32,
    b: u32
}

impl Color{
    pub fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0
        }
    }
}

impl ops::Add<Color> for Color {

    type Output = Color;

    fn add(self, c: Color) -> Color {
        return Color { 
            r: self.r + c.r, 
            g: self.g + c.g, 
            b: self.b + c.b
        };
    }
}

impl ops::Sub<Color> for Color {

    type Output = Color;

    fn sub(self, c: Color) -> Color {
        return Color { 
            r: self.r - c.r, 
            g: self.g - c.g, 
            b: self.b - c.b
        };
    }
}

impl ops::AddAssign<Color> for Color {

    fn add_assign(&mut self, c: Self) {

        *self = Self {
            r: self.r + c.r,
            g: self.g + c.g,
            b: self.b + c.b
        };
    }
}

impl ops::AddAssign<Intensity> for Color {

    fn add_assign(&mut self, i: Intensity) {
        
        *self = Self {
            r: (self.r as f32 + i.er) as u32,
            g: (self.r as f32 + i.eg) as u32,
            b: (self.r as f32 + i.eb) as u32
        };
    }
}

impl ops::Mul<f32> for Color {

    type Output = Color;

    fn mul(self, k: f32) -> Color {
        return Color { 
            r: (self.r as f32 * k) as u32, 
            g: (self.g as f32 * k) as u32, 
            b: (self.b as f32 * k) as u32, 
        };
    }
}

impl ops::Mul<Intensity> for Color {

    type Output = Color;

    fn mul(self, i: Intensity) -> Color {
        return Color { 
            r: (self.r as f32 * i.er) as u32,
            g: (self.r as f32 * i.eg) as u32,
            b: (self.r as f32 * i.eb) as u32
        };
    }
}