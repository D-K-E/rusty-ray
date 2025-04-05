//
use rusty_ray::{
    imgio::{
        imrender::save_pixels,
        imtypes::{Pixel, Point2d},
    },
    selfsync::pipeline::add_to_pipeline,
};
use smol::{
    Executor,
    channel::{Receiver, Sender, unbounded},
    future::block_on,
};
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
    let img_height: u32 = imgrad.img_height;
    let red_: f32 = (x as f32) / (img_width as f32);
    let green_: f32 = (y as f32) / (img_height as f32);
    let blue_: f32 = 0.25;
    let red = red_ * 255.9;
    let green = green_ * 255.9;
    let blue: f32 = blue_ * 255.9;
    let data: image::Rgb<u8> = image::Rgb([red as u8, green as u8, blue as u8]);
    let coord: Point2d = Point2d { x: x, y: y };
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

fn gen_imgraddata<'tasklife>(
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

async fn collect_output<'tasklife>(
    out_r: &'tasklife Receiver<Pixel>,
    quit: &'tasklife Sender<bool>,
    nb_tasks: &'tasklife usize,
) -> Vec<Pixel> {
    let mut result = Vec::<Pixel>::new();
    while result.len() < (*nb_tasks) {
        match out_r.recv().await {
            Ok(pix) => {
                result.push(pix);
            }
            _ => (),
        }
    }
    println!(
        "result filled: result len {}, nb_tasks {}",
        result.len(),
        *nb_tasks
    );
    out_r.close();
    drop(out_r);
    quit.send(true).await;
    drop(quit);
    result
}

fn generate_img_concurrent(img_width: u32, img_height: u32) -> Vec<Pixel> {
    // input channel
    let (q_s, q_r) = unbounded::<bool>();
    let mut ex = Executor::new();
    let (nb_tasks, input_receive) = gen_imgraddata(img_width, img_height, &q_r, &mut ex);
    let bind = input_receive.clone();
    let out_receiver = add_to_pipeline(&q_r, imgrad2pix, bind, &mut ex);
    //
    let result = block_on(ex.run(async { collect_output(&out_receiver, &q_s, &nb_tasks).await }));
    while !ex.is_empty() {
        ex.try_tick();
    }
    // input_send.close();
    // drop(input_send);
    out_receiver.close();
    input_receive.close();
    q_s.close();
    q_r.close();
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
