/// image data type Pixel

use image;

use crate::domain::math3d::constant::real;

#[derive(Clone, Hash, Eq)]
pub struct Point2d {
    pub x: u32,
    pub y: u32,
}

pub struct Pixel {
    data: image::Rgb<u8>,
    coord: Point2d,
}

impl Pixel {
    pub fn new(imdata: image::Rgb<u8>, loc: Point2d) -> Pixel {
        Pixel {
            data: imdata,
            coord: loc,
        }
    }
    pub fn from_rgb(r: real, g: real, b: real, loc: Point2d) -> Pixel {

        let data: image::Rgb<u8> = image::Rgb([r as u8, g as u8, b as u8]);
        Pixel { data: data, coord: loc }

    } 
    pub fn get(self) -> (image::Rgb<u8>, Point2d) {
        (self.data, self.coord)
    }
}
