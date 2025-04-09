/// image factory v1
use crate::data::image::RGBImage;
use crate::data::pixel::Pixel;
use crate::data::imgrad::ImGradientData;
use crate::domain::adapter::imgrad2pix;

pub fn generate_img(img_width: u32, img_height: u32) -> RGBImage {
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
