//! rendering module
use image;

use crate::data::pixel::Pixel;

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
