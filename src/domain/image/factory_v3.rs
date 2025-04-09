//! another image factory: red sphere in blue background

use crate::data::image::RGBImage;
use crate::domain::factory::imgrad_factory;
use crate::domain::adapter::imgrad2ray;
use crate::domain::adapter::ray2pixel_v1;
use crate::domain::selfsync::pipeline::add_to_pipeline;
use crate::domain::selfsync::collect::collect_output;

use smol::{
    Executor,
    channel::unbounded,
    future::block_on
};

pub fn generate_img_concurrent(img_width: u32, img_height: u32) -> RGBImage {
    // input channel
    let (q_s, q_r) = unbounded::<bool>();
    let mut ex = Executor::new();
    let (nb_tasks, input_receive) = imgrad_factory(img_width, img_height, &q_r, &mut ex);
    let bind = input_receive.clone();
    let ray_r = add_to_pipeline(&q_r, imgrad2ray, bind, &mut ex);
    let out_receiver = add_to_pipeline(&q_r, ray2pixel_v1, ray_r, &mut ex);
    //
    // we need to block it here to collect the result
    let result = block_on(ex.run(async { collect_output(out_receiver, q_s, &nb_tasks).await }));
    drop(input_receive);
    result
}
