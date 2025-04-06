//! a simple ray implementation
use crate::math3d::vector::Vec3d;
use std::fmt;

use crate::math3d::constant::real;
use std::fmt::Display;

#[derive(PartialEq)]
struct Ray {
    origin: Vec3d,
    direction: Vec3d,
}

impl Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Ray><origin>{}<origin><direction>{}</direction></Ray>",
            self.origin(),
            self.direction()
        )
    }
}

impl Ray {
    pub fn new(o: Vec3d, d: Vec3d) -> Ray {
        Ray {
            origin: o,
            direction: d,
        }
    }
    pub fn origin(&self) -> Vec3d {
        self.origin.clone()
    }
    pub fn direction(&self) -> Vec3d {
        self.direction.clone()
    }

    pub fn at(&self, coeff: real) -> Vec3d {
        let d = self.direction().scalar_multiply(coeff);
        let new_loc = d.add(&self.origin());
        new_loc
    }
}
