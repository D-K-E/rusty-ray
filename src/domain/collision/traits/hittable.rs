//! hittable object trait

use crate::domain::collision::data::hitinput::HitInput;
use crate::domain::collision::data::hitrecord::HitRecord;
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

pub fn is_hit_task<T: Hittable + Clone + Send>(h: HitInput<T>) -> (HitRecord, bool) {
    let hobj = h.hittable_obj();
    let r = h.ray();
    let min_d = h.min_distance();
    let mx_d = h.max_distance();
    let hrec = HitRecord::null();

    let tpl = hobj.is_hit(r, min_d, mx_d, hrec);
    tpl
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
    let input = HitInput::from_ref(hittable_obj, r, min_distance, max_distance);
    let _ = ex
        .spawn(async move {
            let (hrec, is_h) = is_hit_task(input);

            if is_h {
                let hit_tpl = (hrec, is_h);
                let _ = hit_sender.send(hit_tpl).await;
            }
        })
        .detach();
}
