#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point {
    pub fn dist(&self, p: &Point) -> f32 {
        return ((p.x - self.x) * (p.x - self.x) +
                (p.y - self.y) * (p.y - self.y) + 
                (p.z - self.z) * (p.z - self.z))
                .sqrt();
    }
}