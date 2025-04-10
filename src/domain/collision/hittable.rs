//! hittable object trait

use crate::domain::collision::hitrecord::HitRecord;
use crate::domain::math3d::ray::Ray;
use crate::domain::math3d::constant::real;

pub trait Hittable {

    fn is_hit(&self, r: &Ray,
        min_distance: &real, max_distance: &real,
        hit_rec: HitRecord) -> (HitRecord, bool);

}

