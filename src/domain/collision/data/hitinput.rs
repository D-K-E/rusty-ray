//! collision related data

use crate::domain::collision::traits::hitobject::HitObject;

use crate::domain::math3d::constant::real;
use crate::domain::math3d::ray::Ray;
use std::marker::Send;

#[derive(Clone, Send)]
pub struct HitInput {
    hittable_obj: HitObject,
    ray: Ray,
    min_distance: real,
    max_distance: real,
}

impl HitInput {
    pub fn new(hittable_obj: HitObject, ray: Ray, min_distance: real, max_distance: real) -> Self {
        Self {
            hittable_obj,
            ray,
            min_distance,
            max_distance,
        }
    }
    pub fn from_ref(h: &HitObject, r: &Ray, mn_d: &real, mx_d: &real) -> Self {
        Self {
            hittable_obj: h.clone(),
            ray: r.clone(),
            min_distance: mn_d.clone(),
            max_distance: mx_d.clone(),
        }
    }

    pub fn hittable_obj(&self) -> &HitObject {
        &self.hittable_obj
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    pub fn min_distance(&self) -> &real {
        &self.min_distance
    }

    pub fn max_distance(&self) -> &real {
        &self.max_distance
    }
}
