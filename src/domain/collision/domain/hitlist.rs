//! hittable objects list

use crate::domain::collision::data::hitinput::HitInput;
use crate::domain::collision::data::hitrecord::HitRecord;
use crate::domain::collision::traits::hittable::Hittable;
use crate::domain::math3d::constant::real;
use crate::domain::selfsync::workerpool::spawn_workers;

use crate::domain::math3d::ray::Ray;
use smol::channel::{unbounded, Receiver};

fn hit_concurrent(
    hitlist: &Hittables,
    r: &Ray,
    min_distance: &real,
    max_distance: &real,
    nb_workers: usize,
) -> Receiver<(HitRecord, bool)> {
    (input_s, input_r) = unbounded::<Box<dyn Hittable>>();
    (hitin_s, hit_r) = unbounded::<HitInput<Box<dyn Hittable>>>();
}
