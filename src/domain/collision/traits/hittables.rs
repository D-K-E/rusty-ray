//! hitlist

use crate::domain::math3d::ray::Ray;
use crate::domain::math3d::constant::real;
use crate::domain::collision::traits::hittable::Hittable;
use crate::domain::collision::data::hittables::Hittables;

impl Hittable for Hittables {
    fn is_hit(
        &self,
        r: &Ray,
        min_distance: &real,
        max_distance: &real,
        hit_rec: HitRecord,
    ) -> (HitRecord, bool) {
        let vecs = self.objects();
        let mut tmp_h_rec = hit_rec.clone();
        let mut collision_detected = false;
        for v in vecs.iter() {
            let (h_rec, obj_hit) = v.is_hit(r, min_distance, max_distance, tmp_h_rec.clone());
            if obj_hit {
                let h_rec_dist = h_rec.distance();
                let temp_dist = tmp_h_rec.distance();
                collision_detected = true;
                //
                if temp_dist > h_rec_dist {
                    tmp_h_rec = h_rec;
                }
            }
        }
        (tmp_h_rec, collision_detected)
    }
}
