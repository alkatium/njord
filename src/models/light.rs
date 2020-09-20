use std::ops;
use crate::geometry::point::Point;

pub struct Intensity {  
    pub er: f32,
    pub eg: f32,
    pub eb: f32
}

impl Intensity {
    pub fn default() -> Intensity {
        Intensity {
            er: 0.,
            eg: 0.,
            eb: 0.
        }
    }
}

impl ops::Mul<f32> for Intensity {

    type Output = Intensity;

    fn mul(self, k: f32) -> Intensity {
        return Intensity { 
            er: self.er * k, 
            eg: self.eg * k, 
            eb: self.eb * k
        };
    }
}

pub struct Light {  
    pub position: Point,
    pub intensity: Intensity
}
