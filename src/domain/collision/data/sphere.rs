//! sphere object
use crate::domain::math3d::constant::real;
use crate::domain::math3d::vector::Vec3d;
use std::fmt;
use std::fmt::Display;

pub struct Sphere {
    center: Vec3d,
    radius: real,
}

impl Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Sphere><center>{}<center><radius>{}</radius></Sphere>",
            self.center(),
            self.radius()
        )
    }
}

impl Sphere {
    pub fn new(c: Vec3d, r: real) -> Sphere {
        Sphere {
            center: c,
            radius: r,
        }
    }

    pub fn center(&self) -> Vec3d {
        self.center.clone()
    }

    pub fn radius(&self) -> real {
        self.radius.clone()
    }
}
