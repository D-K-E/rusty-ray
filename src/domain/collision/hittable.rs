//! hittable object trait

use crate::domain::collision::hitrecord::HitRecord;
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

pub struct HitInput<T: Hittable + Clone + Send> {
    hittable_obj: T,
    ray: Ray,
    min_distance: real,
    max_distance: real,
}

impl<T: Hittable + Clone + Send> HitInput<T> {
    pub fn new(hittable_obj: T, ray: Ray, min_distance: real, max_distance: real) -> Self {
        Self {
            hittable_obj,
            ray,
            min_distance,
            max_distance,
        }
    }
    pub fn from_ref(h: &T, r: &Ray, mn_d: &real, mx_d: &real) -> Self {
        Self {
            hittable_obj: h.clone(),
            ray: r.clone(),
            min_distance: mn_d.clone(),
            max_distance: mx_d.clone(),
        }
    }

    pub fn hittable_obj(&self) -> &T {
        &self.hittable_obj
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    pub fn min_distance(&self) -> &real {
        &self.min_distance
    }

    pub fn max_distance(&self) -> &real {
        &self.max_distance
    }
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
    let r_ = r.clone();
    let mn_d = min_distance.clone();
    let mx_d = max_distance.clone();
    let hobj = hittable_obj.clone();
    let _ = ex
        .spawn(async move {
            let (hrec, is_h) = hobj.is_hit(&r_, &mn_d, &mx_d, hit_rec);

            if is_h {
                let hit_tpl = (hrec, is_h);
                hit_sender.send(hit_tpl).await;
            }
        })
        .detach();
}
