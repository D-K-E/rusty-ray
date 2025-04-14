//! hittable objects list

use crate::domain::collision::data::hitrecord::HitRecord;
use crate::domain::collision::data::hitinput::HitInput;
use crate::domain::collision::domain::hittable::Hittable;
use crate::domain::math3d::constant::real;
use crate::domain::selfsync::workerpool::spawn_workers;

use crate::domain::math3d::ray::Ray;
use smol::channel::{unbounded, Receiver};

pub struct Hittables {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn objects(&self) -> &Vec<Box<dyn Hittable>> {
        &self.objects
    }
}

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
