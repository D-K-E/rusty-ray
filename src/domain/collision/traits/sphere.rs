//! traits implementation of sphere
use crate::domain::collision::data::hitrecord::HitRecord;
use crate::domain::collision::data::sphere::Sphere;
use crate::domain::collision::traits::hittable::Hittable;
use crate::domain::math3d::constant::real;
use crate::domain::math3d::ray::Ray;

impl Hittable for Sphere {
    fn is_hit(
        &self,
        r: &Ray,
        min_distance: &real,
        max_distance: &real,
        _hit_rec: HitRecord,
    ) -> (HitRecord, bool) {
        let origin = r.origin();
        let center = self.center();
        let sr = self.radius();
        let direction = r.direction();
        // let dist_origin_to_center = origin.subtract(&center);
        let rd_2 = direction.norm().powi(2);
        let hb = origin.dot(&direction);
        let c = center.norm().powi(2) - (sr * sr);
        let discrim = hb.powi(2) - (rd_2 * c);
        if discrim < 0.0 {
            let hr = HitRecord::null();
            return (hr, false);
        } else {
            let sqd = discrim.sqrt();
            let root = (-hb - sqd) / rd_2;
            let nroot = (-hb + sqd) / rd_2;
            let cond1 = (&root < min_distance) || (max_distance < &root);
            let cond2 = (&nroot < min_distance) || (max_distance < &nroot);
            if cond1 {
                if cond2 {
                    let hr = HitRecord::null();
                    return (hr, false);
                } else {
                    let hit_p = r.at(nroot);
                    let hit_p_sc = hit_p.subtract(&origin);
                    let hnorm = hit_p_sc.scalar_divide(sr);
                    let h_rec = HitRecord::from_ref(&hit_p, &hnorm, &nroot);
                    let h_rec_2 = h_rec.set_face_normal(r, &hnorm);
                    return (h_rec_2, true);
                }
            } else {
                let hit_p = r.at(root);
                let hit_p_sc = hit_p.subtract(&origin);
                let hnorm = hit_p_sc.scalar_divide(sr);
                let h_rec = HitRecord::from_ref(&hit_p, &hnorm, &root);
                let h_rec_2 = h_rec.set_face_normal(r, &hnorm);
                return (h_rec_2, true);
            }
        }
    }
}
