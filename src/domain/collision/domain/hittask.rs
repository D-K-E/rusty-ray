//! hittable task
use crate::domain::{
    collision::{
        data::{hitinput::HitInput, hitobject::HitObject, hitrecord::HitRecord},
        traits::hittable::Hittable,
    },
    math3d::constant::real,
    math3d::ray::Ray,
};
use smol::Executor;
use smol::channel::Sender;

pub fn is_hit_task_v1(h: HitInput) -> (HitRecord, bool) {
    let hobj = h.hittable_obj();
    let r = h.ray();
    let min_d = h.min_distance();
    let mx_d = h.max_distance();
    let hrec = HitRecord::null();

    let tpl = hobj.is_hit(r, min_d, mx_d, hrec);
    tpl
}

pub fn is_hit_task_v2<SideArg1, SideArg2>(
    hp: (HitInput, SideArg1, SideArg2),
) -> ((HitRecord, bool), SideArg1, SideArg2) {
    let (h, p, r) = hp;
    let tpl = is_hit_task_v1(h);
    (tpl, p, r)
}

pub fn spawn_is_hit<'tasklife>(
    hittable_obj: &'tasklife HitObject,
    r: &'tasklife Ray,
    min_distance: &'tasklife real,
    max_distance: &'tasklife real,
    _hit_rec: HitRecord,
    hit_sender: &'tasklife Sender<(HitRecord, bool)>,
    ex: &mut Executor<'tasklife>,
) {
    let input = HitInput::from_ref(hittable_obj, r, min_distance, max_distance);
    let _ = ex
        .spawn(async move {
            let (hrec, is_h) = is_hit_task_v1(input);

            if is_h {
                let hit_tpl = (hrec, is_h);
                let _ = hit_sender.send(hit_tpl).await;
            }
        })
        .detach();
}
