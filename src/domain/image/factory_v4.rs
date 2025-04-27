//! another image factory: red sphere in blue background

use crate::{
    data::{image::RGBImage, pixel::Pixel},
    domain::{
        adapter::{hit_record2pixel_info, imgrad2ray},
        collision::domain::concurrent_hit::hit_concurrent_v2,
        factory::{imgrad_factory, world_v1},
        math3d::constant::real,
        selfsync::{collect::collect_output, pipeline::add_to_pipeline},
    },
};

use smol::{Executor, channel::unbounded, future::block_on};
use std::collections::HashMap;

pub fn generate_img_concurrent(img_width: u32, img_height: u32) -> RGBImage {
    // input channel
    let (q_s, q_r) = unbounded::<bool>();
    let mut ex = Executor::new();
    let (nb_tasks, input_receive) = imgrad_factory(img_width, img_height, &q_r, &mut ex);
    let bind = input_receive.clone();

    println!("imgrad fac");
    //
    let ray_receiver = add_to_pipeline(&q_r, imgrad2ray, bind, &mut ex);
    println!("created ray receiver");
    //
    // create world
    let world = world_v1();

    let min_d: real = 0.0;
    let max_d: real = real::MAX;

    let hitrec_receiver = hit_concurrent_v2(world, min_d, max_d, 3, ray_receiver, &q_r, &mut ex);
    println!("created hitrec receiver");

    //
    let pixel_receiver = add_to_pipeline(&q_r, hit_record2pixel_info, hitrec_receiver, &mut ex);
    println!("created pixel receiver");
    //
    // we need to block it here to collect the result
    let result: Vec<(Pixel, (bool, real))> =
        block_on(ex.run(async { collect_output(pixel_receiver, q_s, &nb_tasks).await }));
    println!("vec pixels");

    drop(input_receive);

    // now filter vec for false
    let hit_pixels: Vec<(Pixel, (bool, real))> = result
        .into_iter()
        .filter(|n| {
            let (_, (is_h, _)) = n;
            *is_h
        })
        .collect();
    // collect into hash map
    let mut hmap = HashMap::new();
    for hit_pix in hit_pixels.into_iter() {
        //
        let (pix, (_, dist)) = hit_pix;
        let (im, loc) = pix.get();
        match hmap.get(&loc) {
            Some((_, dist_)) => {
                if dist < (*dist_) {
                    hmap.insert(loc, (im, dist));
                }
            }
            None => {
                hmap.insert(loc, (im, dist));
            }
        }
    }
    // populate a vec of pixels from hashmap
    let pixels = hmap
        .into_iter()
        .map(|(loc_, (im_, _))| Pixel::new(im_, loc_))
        .collect();
    pixels
}
