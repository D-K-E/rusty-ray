//! hittable object trait

use crate::domain::collision::data::hitrecord::HitRecord;
use crate::domain::math3d::constant::real;
use crate::domain::math3d::ray::Ray;

pub trait Hittable {
    fn is_hit(
        &self,
        r: &Ray,
        min_distance: &real,
        max_distance: &real,
        hit_rec: HitRecord,
    ) -> (HitRecord, bool);
}
