//! collision related data

use crate::domain::collision::domain::hittable::Hittable;

use crate::domain::math3d::ray::Ray;
use crate::domain::math3d::constant::real;
use std::marker::Send;

pub struct HitInput<T: Hittable + Clone + Send> {
    hittable_obj: T,
    ray: Ray,
    min_distance: real,
    max_distance: real,
}

impl<T: Hittable + Clone + Send> HitInput<T> {
    pub fn new(hittable_obj: T, ray: Ray, min_distance: real, max_distance: real) -> Self {
        Self {
            hittable_obj,
            ray,
            min_distance,
            max_distance,
        }
    }
    pub fn from_ref(h: &T, r: &Ray, mn_d: &real, mx_d: &real) -> Self {
        Self {
            hittable_obj: h.clone(),
            ray: r.clone(),
            min_distance: mn_d.clone(),
            max_distance: mx_d.clone(),
        }
    }

    pub fn hittable_obj(&self) -> &T {
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
