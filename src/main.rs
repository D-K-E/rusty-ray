//
use image;

struct Point2d {
    x: u32,
    y: u32,
}

struct Pixel {
    data: image::Rgb<u8>,
    coord: Point2d,
}

fn render_img(img_width: u32, img_height: u32, pixels: Vec<Pixel>, path: String) {
    let mut imgbuf = image::ImageBuffer::new(img_width, img_height);
    for pix in pixels {
        let data = pix.data;
        let coord = pix.coord;
        let pixel = imgbuf.get_pixel_mut(coord.x, coord.y);
        *pixel = data;
    }
    imgbuf.save(path).unwrap();
}

fn generate_img(img_width: u32, img_height: u32) -> Vec<Pixel> {
    let mut result = Vec::<Pixel>::new();
    for x in 0..img_width {
        for y in 0..img_height {
            let red_ = (x as f32) / (img_width as f32);
            let green_ = (y as f32) / (img_height as f32);
            let blue_: f32 = 0.25;
            let red = red_ * 255.9;
            let green = green_ * 255.9;
            let blue = blue_ * 255.9;
            let data = image::Rgb([red as u8, green as u8, blue as u8]);
            let coord = Point2d { x: x, y: y };
            let pix = Pixel {
                data: data,
                coord: coord,
            };
            result.push(pix);
        }
    }
    result
}

fn main() {
    let im_width: u32 = 256;
    let im_height: u32 = 256;
    let pixels = generate_img(im_width, im_height);
    render_img(im_width, im_height, pixels, "test.png".to_string());
}
