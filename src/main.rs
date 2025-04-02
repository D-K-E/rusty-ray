//
use rusty_ray::imgio::imrender::save_pixels;
use rusty_ray::imgio::imtypes::Pixel;
use rusty_ray::imgio::imtypes::Point2d;
use rusty_ray::selfsync::pipeline::add_to_pipeline;
use smol::channel::Sender;
use smol::channel::unbounded;
use smol::future::race;
use std::thread::scope;
// use std::future::{Future, FutureExt};

struct ImGradientData {
    x: u32,
    y: u32,
    img_width: u32,
    img_height: u32,
}
impl ImGradientData {
    fn new(x: u32, y: u32, w: u32, h: u32) -> ImGradientData {
        ImGradientData {
            x: x,
            y: y,
            img_width: w,
            img_height: h,
        }
    }
}

fn imgrad2pix(imgrad: ImGradientData) -> Pixel {
    let x = imgrad.x;
    let y = imgrad.y;
    let img_width = imgrad.img_width;
    let img_height = imgrad.img_height;
    let red_ = (x as f32) / (img_width as f32);
    let green_ = (y as f32) / (img_height as f32);
    let blue_: f32 = 0.25;
    let red = red_ * 255.9;
    let green = green_ * 255.9;
    let blue = blue_ * 255.9;
    let data = image::Rgb([red as u8, green as u8, blue as u8]);
    let coord = Point2d { x: x, y: y };
    Pixel::new(data, coord)
}

fn generate_img(img_width: u32, img_height: u32) -> Vec<Pixel> {
    let mut result = Vec::<Pixel>::new();
    for x in 0..img_width {
        for y in 0..img_height {
            let imgrad = ImGradientData::new(x, y, img_width, img_height);
            let pix = imgrad2pix(imgrad);
            result.push(pix);
        }
    }
    result
}

fn process_image_gradient(imw: u32, imh: u32, input_send: Sender<ImGradientData>) -> () {
    for x in 0..imw {
        for y in 0..imh {
            let imgrad = ImGradientData::new(x, y, imw, imh);
            input_send.send(imgrad).await;
        }
    }
    drop(input_send);
}

fn generate_img_concurrent(img_width: &u32, img_height: &u32) -> Vec<Pixel> {
    // input channel
    let (input_send, input_receive) = unbounded::<ImGradientData>();
    let (quit_send, quit_receive) = unbounded::<bool>();
    println!("created quit and input channels");
    let pixel_receive = add_to_pipeline(&quit_receive, imgrad2pix, &input_receive);
    println!("created pixel receive");

    scope(|scope| {
        scope.spawn(move |_| {
            process_image_gradient(img_width, img_height, input_send);
            // .boxed()
        });
        // notice that we are not returning the child scope join handler
        // by adding the semicolumn
    })
    .unwrap();
    println!("sent all data");

    // collect from output channel to result
    let mut result = Vec::<Pixel>::new();
    loop {
        match pixel_receive.recv() {
            Ok(pix) => {
                result.push(pix);
            }
            Err(_) => break,
        }
    }
    result
}

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
    let pixels = generate_img_concurrent(&im_width, &im_height);
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
