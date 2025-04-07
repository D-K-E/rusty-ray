/// camera data module
use crate::math3d::constant::real;
use crate::math3d::vector::Vec3d;

pub fn default_aspect_ratio() -> real {
    16.0 / 9.0
}

pub fn default_view_port_height() -> real {
    2.0
}

pub fn default_focal_length() -> real {
    1.0
}

pub fn default_view_port_width() -> real {
    default_aspect_ratio() * default_view_port_height()
}

pub fn default_camera_origin() -> Vec3d {
    Vec3d::from_scalar(0.0)
}

pub fn default_camera_height() -> Vec3d {
    Vec3d::from_xyz(default_view_port_width(), 0.0, 0.0)
}

pub fn default_camera_v() -> Vec3d {
    Vec3d::from_xyz(0.0, default_view_port_height(), 0.0)
}

pub fn default_lower_left_corner() -> Vec3d {
    let fvec = Vec3d::from_xyz(0.0, 0.0, default_focal_length());
    let vhalf = default_camera_v().scalar_multiply(0.5);
    let hhalf = default_camera_height().scalar_multiply(0.5);
    let origin_min_half = default_camera_origin().subtract(&hhalf);
    let origin_min_v = origin_min_half.subtract(&vhalf);
    origin_min_v.subtract(&fvec)
}
