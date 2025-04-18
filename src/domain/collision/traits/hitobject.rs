//! traits implementation of hit object

use crate::domain::collision::data::hitobject::HitObject;
use crate::domain::collision::data::hitobject::ObjectKind;
use crate::domain::collision::data::hitrecord::HitRecord;
use crate::domain::collision::traits::hittable::Hittable;
use crate::domain::math3d::constant::real;
use crate::domain::math3d::ray::Ray;

impl Hittable for HitObject {
    fn is_hit(
        &self,
        r: &Ray,
        min_distance: &real,
        max_distance: &real,
        hit_rec: HitRecord,
    ) -> (HitRecord, bool) {
        if self.kind() == ObjectKind::Sphere {
            let s = self.to_sphere();
            let tpl = s.is_hit(r, min_distance, max_distance, hit_rec);
            return tpl;
        } else {
            panic!("unexpected kind");
        }
    }
}
