/// adapter objects
use crate::data::imgrad::ImGradientData;
use crate::data::pixel::{Pixel, Point2d};
use crate::domain::camera::camdata::{
    default_camera_height, default_camera_origin, default_camera_v, default_lower_left_corner,
};
use crate::domain::math3d::constant::real;
use crate::domain::math3d::ray::Ray;
use image;

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
    //
    let c_origin = default_camera_origin();
    let vvert = default_camera_v().scalar_multiply(v);
    let uhor = default_camera_height().scalar_multiply(u);
    let vv_min_or = vvert.subtract(&c_origin);
    let uhor_plus_vv = uhor.add(&vv_min_or);
    let llc_plus_uhor = default_lower_left_corner().add(&uhor_plus_vv);
    Ray::new(c_origin, llc_plus_uhor)
}
