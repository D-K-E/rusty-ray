//! another image factory: red sphere in blue background

use crate::data::image::RGBImage;
use crate::data::pixel::Pixel;
use crate::domain::adapter::hit_record2pixel_info;
use crate::domain::adapter::imgrad2ray;
use crate::domain::collision::domain::concurrent_hit::hit_concurrent_v2;
use crate::domain::factory::imgrad_factory;
use crate::domain::factory::world_v1;
use crate::domain::math3d::constant::real;
use crate::domain::selfsync::collect::collect_output;
use crate::domain::selfsync::pipeline::add_to_pipeline;
use crate::domain::selfsync::pump_value::pump_value_to_channel;

use smol::{Executor, channel::unbounded, future::block_on};
use std::collections::HashMap;

pub fn generate_img_concurrent(img_width: u32, img_height: u32) -> RGBImage {
    // input channel
    let (q_s, q_r) = unbounded::<bool>();
    let mut ex = Executor::new();
    let (nb_tasks, input_receive) = imgrad_factory(img_width, img_height, &q_r, &mut ex);
    let bind = input_receive.clone();

    //
    let ray_receiver = add_to_pipeline(&q_r, imgrad2ray, bind, &mut ex);
    //
    // create world
    let world = world_v1();

    let min_d: real = 0.0;
    let max_d: real = real::MAX;

    let hitrec_receiver =
        hit_concurrent_v2(&world, &min_d, &max_d, 3, &ray_receiver, &q_r, &mut ex);

    //
    let pixel_receiver = add_to_pipeline(&q_r, hit_record2pixel_info, hitrec_receiver, &mut ex);
    //
    // we need to block it here to collect the result
    let result: Vec<(Pixel, (bool, real))> =
        block_on(ex.run(async { collect_output(pixel_receiver, q_s, &nb_tasks).await }));

    drop(input_receive);

    // now filter vec for false
    let hit_pixels: Vec<(Pixel, (bool, real))> = result
        .into_iter()
        .filter(|&n| {
            let (p, (is_h, distance)) = n;
            is_h
        })
        .collect();
    // collect into hash map
    let mut hmap = HashMap::new();
    for hit_pix in hit_pixels.into_iter() {
        //
        let (pix, (is_hit, dist)) = hit_pix;
        let (im, loc) = pix.get();
        match hmap.get(&loc) {
            Some((im_, dist_)) => {
                if dist < dist_ {
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
        .map(|(loc_, (im_, d))| Pixel::new(im_, loc_))
        .collect();
    pixels
}
