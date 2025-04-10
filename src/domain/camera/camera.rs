//! camera type

use crate::domain::camera::camdata::{
    default_camera_height, default_camera_origin, default_camera_v, default_lower_left_corner,
};
use crate::domain::math3d::constant::real;
use crate::domain::math3d::ray::Ray;
use crate::domain::math3d::vector::Vec3d;
use std::fmt::Display;

#[derive(PartialEq, Display)]
pub struct Camera {
    origin: Vec3d,
    lower_left_corner: Vec3d,
    horizontal: Vec3d,
    vertical: Vec3d,
}

impl Camera {
    pub fn default_camera() -> Camera {
        let orig = default_camera_origin();
        let horiz = default_camera_height();
        let vert = default_camera_v();
        let lower_left = default_lower_left_corner();
        Camera {
            origin: orig,
            horizontal: horiz,
            vertical: vert,
            lower_left_corner: lower_left,
        }
    }

    pub fn ray_from_uv(&self, u: real, v: real) -> Ray {
        let vvert = self.vertical().scalar_multiply(v);
        let uhor = self.horizontal().scalar_multiply(u);
        let vv_min_or = vvert.subtract(self.origin());
        let uhor_plus_vv = uhor.add(&vv_min_or);
        let llc_plus_uhor = self.lower_left_corner().add(&uhor_plus_vv);
        Ray::new(self.origin().clone(), llc_plus_uhor)
    }

    pub fn origin(&self) -> &Vec3d {
        &self.origin
    }

    pub fn lower_left_corner(&self) -> &Vec3d {
        &self.lower_left_corner
    }

    pub fn horizontal(&self) -> &Vec3d {
        &self.horizontal
    }

    pub fn vertical(&self) -> &Vec3d {
        &self.vertical
    }
}
