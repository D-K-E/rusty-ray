//
use rusty_ray::{
    imgio::imrender::save_pixels,
    imgrad::utils::{generate_img, generate_img_concurrent},
};
// use std::future::{Future, FutureExt};

fn sequential_main() {
    let im_width: u32 = 256;
    let im_height: u32 = 256;
    let pixels = generate_img(im_width, im_height);
    save_pixels(
        im_width,
        im_height,
        pixels,
        "assets/test_seq.png".to_string(),
    );
    println!("done")
}

fn concurrent_main() {
    let im_width: u32 = 256;
    let im_height: u32 = 256;
    let pixels = generate_img_concurrent(im_width, im_height);
    save_pixels(
        im_width,
        im_height,
        pixels,
        "assets/test_concurrent.png".to_string(),
    );
    println!("done")
}

fn main() {
    // sequential_main();
    concurrent_main();
}
