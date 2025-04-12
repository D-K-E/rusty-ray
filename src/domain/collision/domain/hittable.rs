//! hittable object trait

use crate::domain::collision::data::hitrecord::HitRecord;
use crate::domain::collision::data::hitinput::HitInput;
use crate::domain::math3d::constant::real;
use crate::domain::math3d::ray::Ray;
use smol::Executor;
use smol::channel::Sender;
use std::marker::Send;

pub trait Hittable {
    fn is_hit(
        &self,
        r: &Ray,
        min_distance: &real,
        max_distance: &real,
        hit_rec: HitRecord,
    ) -> (HitRecord, bool);
}



pub fn spawn_is_hit<'tasklife, T: Hittable + Clone + Send>(
    hittable_obj: &'tasklife T,
    r: &'tasklife Ray,
    min_distance: &'tasklife real,
    max_distance: &'tasklife real,
    hit_rec: HitRecord,
    hit_sender: &'tasklife Sender<(HitRecord, bool)>,
    ex: &mut Executor<'tasklife>,
) {
    let input = HitInput::from_ref(hittable_obj,
        r, min_distance, max_distance);
    let _ = ex
        .spawn(async move {
            let (hrec, is_h) = hobj.is_hit(&r_, &mn_d, &mx_d, hit_rec);

            if is_h {
                let hit_tpl = (hrec, is_h);
                let _ = hit_sender.send(hit_tpl).await;
            }
        })
        .detach();
}
