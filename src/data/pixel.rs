/// image data type Pixel

use image;

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
    pub fn get(self) -> (image::Rgb<u8>, Point2d) {
        (self.data, self.coord)
    }
}
