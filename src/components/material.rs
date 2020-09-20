use crate::components::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ka: f32,
    pub kd: f32,
    pub ks: f32,
    pub s: f32
}

impl Material {

    pub fn default() -> Material {
        Material {
            color: Color {
                r: 0.8,
                g: 0.8,
                b: 0.8
            },
            ka: 0.5,
            kd: 0.5,
            ks: 0.5,
            s: 10.
        }
    }
}