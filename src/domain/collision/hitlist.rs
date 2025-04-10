//! hittable objects list

use crate::domain::collision::hittable::Hittable;

pub struct Hittables {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }
}

impl Hittable for Hittables {
}
