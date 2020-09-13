use crate::geometry::point::Point;

pub struct Intensity {  
    er: f32,
    ev: f32,
    eb: f32
}

pub struct Light {  
    position: Point,
    intensity: Intensity
}
