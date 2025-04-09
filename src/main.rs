//
use rusty_ray::{
    present::imrender::save_pixels,
    domain::image::factory_v1::generate_img,
    domain::image::factory_v2::generate_img_concurrent as gen_img_concurrent_v2,
    domain::image::factory_v3::generate_img_concurrent as gen_img_concurrent_v3,
    domain::math3d::constant::real,
};
// use std::future::{Future, FutureExt};

fn sequential_main_v1() {
    let im_width: u32 = 256;
    let aspect_ratio = 16.0 / 9.0;
    let im_height = ((im_width as real) / aspect_ratio).round() as u32;
    let pixels = generate_img(im_width, im_height);
    save_pixels(
        im_width,
        im_height,
        pixels,
        "assets/test_v1.png".to_string(),
    );
    println!("done")
}

fn concurrent_main_v2() {
    let im_width: u32 = 256;
    let aspect_ratio = 16.0 / 9.0;
    let im_height = ((im_width as real) / aspect_ratio).round() as u32;
    let pixels = gen_img_concurrent_v2(im_width, im_height);
    save_pixels(
        im_width,
        im_height,
        pixels,
        "assets/test_v2.png".to_string(),
    );
    println!("done")
}

fn concurrent_main_v3() {
    let im_width: u32 = 256;
    let aspect_ratio = 16.0 / 9.0;
    let im_height = ((im_width as real) / aspect_ratio).round() as u32;
    let pixels = gen_img_concurrent_v3(im_width, im_height);
    save_pixels(
        im_width,
        im_height,
        pixels,
        "assets/test_v3.png".to_string(),
    );
    println!("done")
}

fn main() {
    // sequential_main_v1();
    // concurrent_main_v2();
    concurrent_main_v3();
}
