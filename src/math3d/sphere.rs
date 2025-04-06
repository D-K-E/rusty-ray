//! sphere object
use crate::math3d::constant::real;
use crate::math3d::ray::Ray;
use crate::math3d::vector::Vec3d;
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

    pub fn is_hit(&self, r: &Ray) -> bool {
        let origin = r.origin();
        let center = self.center();
        let direction = r.direction();
        let dist_origin_to_center = origin.subtract(&center);
        let discrim_a = direction.norm().powi(2);
        let discrim_b = dist_origin_to_center.dot(&direction) * 2.0;
        let discrim_c = dist_origin_to_center.norm().powi(2) - self.radius.powi(2);
        let discrim = discrim_b.powi(2) - (4.0 * discrim_a * discrim_c);
        discrim > 0.0
    }
}
