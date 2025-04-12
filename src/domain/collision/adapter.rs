//! collision adapters

use crate::domain::collision::hitrecord::HitRecord;
use crate::domain::collision::hittable::HitInput;
use crate::domain::collision::hittable::Hittable;
use std::marker::Send;

pub fn is_hit<T: Hittable+Clone+Send>(h: HitInput<T>) -> (HitRecord, bool) {
    let hobj = h.hittable_obj();
    let r = h.ray();
    let min_d = h.min_distance();
    let mx_d = h.max_distance();
    let hrec = HitRecord::null();

    let tpl = hobj.is_hit(r, min_d, mx_d, hrec);
    tpl
}
