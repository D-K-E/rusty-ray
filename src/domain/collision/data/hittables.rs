//! hittables

use crate::domain::collision::data::hitobject::HitObject;
use smol::channel::{Receiver, unbounded};

pub struct Hittables {
    objects: Vec<HitObject>,
}

impl Hittables {
    pub fn new(objects: Vec<HitObject>) -> Self {
        Self { objects }
    }

    pub fn objects(&self) -> &[HitObject] {
        &self.objects
    }
}
