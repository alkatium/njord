use crate::components::color::Color;

#[derive(Debug)]
pub struct Material {
    color: Color,
    kd: f32,
    ks: f32,
    s: f32
}