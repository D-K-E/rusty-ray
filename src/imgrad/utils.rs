//
use crate::{
    imgio::{
        imrender::collect_output,
        imtypes::{Pixel, Point2d},
    },
    imgrad::types::ImGradientData,
    math3d::ray::Ray,
    math3d::constant::real,
    selfsync::pipeline::add_to_pipeline,
};
use smol::{
    Executor,
    channel::{Receiver, unbounded},
    future::block_on,
};

pub fn imgrad2pix(imgrad: ImGradientData) -> Pixel {
    let x = imgrad.x();
    let y = imgrad.y();
    let img_width = imgrad.img_width();
    let img_height: u32 = imgrad.img_height();
    let red_: real = (x as real) / (img_width as real);
    let green_: real = (y as real) / (img_height as real);
    let blue_: real = 0.25;
    let red = red_ * 255.9;
    let green = green_ * 255.9;
    let blue: real = blue_ * 255.9;
    let data: image::Rgb<u8> = image::Rgb([red as u8, green as u8, blue as u8]);
    let coord: Point2d = Point2d { x, y };
    Pixel::new(data, coord)
}

pub fn imgrad2ray(imgrad: ImGradientData) -> Ray {
    //
    let x = imgrad.x();
    let y = imgrad.y();
    let img_width = imgrad.img_width();
    let img_height: u32 = imgrad.img_height();
    let u: real = (x as real) / (img_width as real);
    let v: real = (y as real) / (img_height as real);

}

pub fn generate_img(img_width: u32, img_height: u32) -> Vec<Pixel> {
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

pub fn gen_imgraddata<'tasklife>(
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

pub fn generate_img_concurrent(img_width: u32, img_height: u32) -> Vec<Pixel> {
    // input channel
    let (q_s, q_r) = unbounded::<bool>();
    let mut ex = Executor::new();
    let (nb_tasks, input_receive) = gen_imgraddata(img_width, img_height, &q_r, &mut ex);
    let bind = input_receive.clone();
    let out_receiver = add_to_pipeline(&q_r, imgrad2pix, bind, &mut ex);
    //
    // we need to block it here to collect the result
    let result = block_on(ex.run(async { collect_output(out_receiver, q_s, &nb_tasks).await }));
    drop(input_receive);
    result
}
