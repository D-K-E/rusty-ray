//! hittables

use crate::domain::collision::traits::hittable::Hittable;
use crate::domain::math3d::ray::Ray;
use smol::channel::{unbounded, Receiver};

pub struct Hittables {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn objects(&self) -> &Vec<Box<dyn Hittable>> {
        &self.objects
    }
}


