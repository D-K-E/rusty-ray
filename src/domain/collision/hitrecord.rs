//! a record object to hold info regarding collided objects

use crate::domain::math3d::vector::Vec3d;
use crate::domain::math3d::constant::real;

pub struct HitRecord {
    point: Vec3d,
    normal: Vec3d,
    distance: real
}

impl HitRecord {
    pub fn new(point: Vec3d, normal: Vec3d, distance: real) -> Self {
        Self { point, normal, distance }
    }
    pub fn null() -> Self {
        let d = real::MAX;
        let n = Vec3d::from_scalar(real::MAX);
        let p = Vec3d::from_scalar(real::MAX);
        HitRecord {point: p, normal: n, distance: d}
    }

    pub fn point(&self) -> &Vec3d {
        &self.point
    }

    pub fn normal(&self) -> &Vec3d {
        &self.normal
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }
}


