//! a record object to hold info regarding collided objects

use crate::domain::math3d::vector::Vec3d;
use crate::domain::math3d::ray::Ray;
use crate::domain::math3d::constant::real;

#[derive(Clone)]
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

    pub fn distance(&self) -> real {
        self.distance
    }

    pub fn set_face_normal(&self, r: &Ray, ov: &Vec3d) -> Self {
        //
        let rd = r.direction();
        let face_cond = rd.dot(&ov);
        let is_cond = face_cond < 0.0;
        if is_cond {
            HitRecord::new(
                self.point().clone(),
                ov.clone(),
                self.distance())
        }else{
            HitRecord::new(
                self.point().clone(),
                ov.scalar_multiply(-1.0),
                self.distance())

        }
    }
}



