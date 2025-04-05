//
pub struct ImGradientData {
    x: u32,
    y: u32,
    img_width: u32,
    img_height: u32,
}
impl ImGradientData {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> ImGradientData {
        ImGradientData {
            x: x,
            y: y,
            img_width: w,
            img_height: h,
        }
    }

    pub fn img_width(&self) -> u32 {
        self.img_width
    }

    pub fn img_height(&self) -> u32 {
        self.img_height
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn x(&self) -> u32 {
        self.x
    }
}
