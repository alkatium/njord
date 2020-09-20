use std::ops;

use crate::models::light::Intensity;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color{
    pub fn white() -> Color {
        Color {
            r: 1.,
            g: 1.,
            b: 1.
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0.,
            g: 0.,
            b: 0.
        }
    }

    pub fn to_rgb(&self) -> Color {
        Color {
            r: self.r * 255.,
            g: self.g * 255.,
            b: self.b * 255.
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
            r: self.r + i.er,
            g: self.r + i.eg,
            b: self.r + i.eb
        };
    }
}

impl ops::Mul<&Intensity> for Color {

    type Output = Color;

    fn mul(self, i: &Intensity) -> Color {
        return Color { 
            r: self.r * i.er,
            g: self.r * i.eg,
            b: self.r * i.eb
        };
    }
}

impl ops::Mul<Intensity> for Color {

    type Output = Color;

    fn mul(self, i: Intensity) -> Color {
        return Color { 
            r: self.r * i.er,
            g: self.r * i.eg,
            b: self.r * i.eb
        };
    }
}

impl ops::Mul<f32> for Color {

    type Output = Color;

    fn mul(self, k: f32) -> Color {
        return Color { 
            r: self.r * k, 
            g: self.g * k, 
            b: self.b * k, 
        };
    }
}