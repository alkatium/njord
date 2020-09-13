use crate::shapes::object::Object;

pub struct Intersection {
    pub object: Box<dyn Object>,
    pub distance: f32,
}