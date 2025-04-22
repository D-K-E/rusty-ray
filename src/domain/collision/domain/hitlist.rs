//! hittable objects list

use crate::domain::collision::data::{
    hitinput::HitInput, hitrecord::HitRecord, hittables::Hittables,
};
use crate::domain::collision::domain::hittask::is_hit_task;
use crate::domain::selfsync::workerpool::spawn_workers;
use smol::Executor;

use crate::domain::math3d::constant::real;
use crate::domain::math3d::ray::Ray;
use smol::channel::{Receiver, unbounded};


fn hit_concurrent<'tasklife>(
    hitlist: &Hittables,
    r: &Ray,
    min_distance: &real,
    max_distance: &real,
    nb_workers: usize,
    quit: &'tasklife Receiver<bool>,
    ex: &mut Executor<'tasklife>,
) -> Receiver<(HitRecord, bool)> {
    let (input_s, input_r) = unbounded::<HitInput>();
    let out_r = spawn_workers(nb_workers, quit, is_hit_task, input_r, ex);

    // start pumping hitinput through input_r
    for hit_object in hitlist.objects() {
        //
        let h_in = HitInput::from_ref(hit_object, r, min_distance, max_distance);
        //
        let send_clone = input_s.clone();
        let _ = ex.spawn(async move {
            let _ = send_clone.send(h_in).await;
        });
    }
    out_r
}

