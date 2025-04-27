//! various factory function
use crate::data::imgrad::ImGradientData;
use crate::domain::collision::data::hitobject::HitObject;
use crate::domain::collision::data::hittables::Hittables;
use crate::domain::collision::data::sphere::Sphere;
use crate::domain::math3d::vector::Vec3d;

use smol::{
    Executor,
    channel::{Receiver, unbounded},
};

pub fn imgrad_factory<'tasklife>(
    imw: u32,
    imh: u32,
    quit: &'tasklife Receiver<bool>,
    ex: &mut Executor<'tasklife>,
) -> (usize, Receiver<ImGradientData>) {
    let tw = imw.clone();
    let th = imh.clone();
    let nb_tasks: usize = (imw * imh) as usize;
    let (input_send, input_receive) = unbounded::<ImGradientData>();
    let _t = ex
        .spawn(async move {
            for x in 0..imw {
                for y in 0..imh {
                    match quit.try_recv() {
                        Ok(_) => break,
                        _ => (),
                    }
                    let imgrad = ImGradientData::new(x, y, tw, th);
                    let _ = input_send.send(imgrad).await;
                }
            }
            drop(input_send);
            println!("input_send closed");
        })
        .detach();
    println!("finished sending");
    return (nb_tasks, input_receive);
}

pub fn world_v1() -> Hittables {
    let c1 = Vec3d::from_xyz(0.0, 0.0, -1.0);
    let c2 = Vec3d::from_xyz(0.0, -100.5, -1.0);
    let s1 = Sphere::new(c1, 0.5);
    let s2 = Sphere::new(c2, 100.0);
    let h1 = HitObject::from_sphere(&s1);
    let h2 = HitObject::from_sphere(&s2);
    let hit_objs = vec![h1, h2];
    let hits = Hittables::new(hit_objs);
    hits
}
