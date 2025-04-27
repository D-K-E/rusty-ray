//! hittable objects list

use crate::data::pixel::Point2d;
use crate::domain::collision::data::{
    hitinput::HitInput, hitrecord::HitRecord, hittables::Hittables,
};
use crate::domain::collision::domain::hittask::is_hit_task_v1;
use crate::domain::collision::domain::hittask::is_hit_task_v2;
use crate::domain::selfsync::workerpool::spawn_workers;
use smol::Executor;

use crate::domain::math3d::constant::real;
use crate::domain::math3d::ray::Ray;
use smol::channel::{Receiver, TryRecvError, unbounded};

pub fn hit_concurrent_v1<'tasklife>(
    hitlist: &Hittables,
    r: &Ray,
    min_distance: &real,
    max_distance: &real,
    nb_workers: usize,
    quit: &'tasklife Receiver<bool>,
    ex: &mut Executor<'tasklife>,
) -> Receiver<(HitRecord, bool)> {
    let (input_s, input_r) = unbounded::<HitInput>();
    let out_r = spawn_workers(nb_workers, quit, is_hit_task_v1, input_r, ex);

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

pub fn hit_concurrent_v2<'tasklife>(
    hitlist: Hittables,
    min_distance: real,
    max_distance: real,
    nb_workers: usize,
    ray_receiver: Receiver<(Ray, Point2d)>,
    quit: &'tasklife Receiver<bool>,
    ex: &mut Executor<'tasklife>,
) -> Receiver<((HitRecord, bool), Point2d, Ray)> {
    let (input_s, input_r) = unbounded::<(HitInput, Point2d, Ray)>();
    let out_r = spawn_workers(
        nb_workers,
        quit,
        is_hit_task_v2::<Point2d, Ray>,
        input_r,
        ex,
    );

    let _ = ex.spawn(async move {
        let min_d = min_distance;
        let max_d = max_distance;
        let hits = hitlist;
        let ray_r = ray_receiver;
        loop {
            if quit.is_closed() {
                break;
            }
            match ray_r.try_recv() {
                Ok(r_tpl) => {
                    let send_clone = input_s.clone();
                    let (r, p) = r_tpl;
                    for hit_object in hits.objects() {
                        let h_in = HitInput::from_ref(hit_object, &r, &min_d, &max_d);
                        //
                        let h_tpl = (h_in, p.clone(), r.clone());
                        let _ = send_clone.send(h_tpl).await;
                        println!("sent hit input");
                    }
                }
                Err(TryRecvError::Closed) => break,
                Err(TryRecvError::Empty) => (),
            }
        }
    });

    out_r
}
