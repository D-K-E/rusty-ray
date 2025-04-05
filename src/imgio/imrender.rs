//! rendering module
use image;

use crate::imgio::imtypes::Pixel;
use smol::{
    channel::{Receiver, Sender},
    stream::StreamExt,
};

/// simple function to render images to a buffer
pub fn save_pixels(img_width: u32, img_height: u32, pixels: Vec<Pixel>, path: String) {
    let mut imgbuf = image::ImageBuffer::new(img_width, img_height);
    for pix in pixels {
        let (data, coord) = pix.get();
        let pixel = imgbuf.get_pixel_mut(coord.x, coord.y);
        *pixel = data;
    }
    imgbuf.save(path).unwrap();
}

pub async fn collect_output<'tasklife>(
    out_r: Receiver<Pixel>,
    quit: Sender<bool>,
    nb_tasks: &'tasklife usize,
) -> Vec<Pixel> {
    let result: Vec<Pixel> = out_r.collect().await;
    println!(
        "result filled: result len {}, nb_tasks {}",
        result.len(),
        *nb_tasks
    );
    let _ = quit.send(true).await;
    let _ = drop(quit);
    result
}
