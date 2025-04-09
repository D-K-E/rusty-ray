//! basic vector related stuff

use crate::domain::math3d::constant::real;
use nalgebra::base::Vector3;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Vec3d {
    data: Vector3<real>,
}
impl Display for Vec3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Vec3d><x>{}</x><y>{}</y><z>{}</z></Vec3d>",
            self.x(),
            self.y(),
            self.z()
        )
    }
}
impl Vec3d {
    pub fn new(v: Vector3<real>) -> Vec3d {
        Vec3d { data: v }
    }
    pub fn from_scalar(v: real) -> Vec3d {
        Vec3d {
            data: Vector3::new(v, v, v),
        }
    }

    pub fn from_xyz(x: real, y: real, z: real) -> Vec3d {
        Vec3d {
            data: Vector3::new(x, y, z),
        }
    }
    pub fn data(&self) -> Vector3<real> {
        self.data.clone()
    }
    pub fn x(&self) -> real {
        self.data.x.clone()
    }
    pub fn y(&self) -> real {
        self.data.y.clone()
    }

    pub fn z(&self) -> real {
        self.data.z.clone()
    }
    pub fn to_decimal_array(&self) -> [Decimal; 3] {
        let darr: [String; 3] = [
            self.x().to_string(),
            self.y().to_string(),
            self.z().to_string(),
        ];
        let mut xyz: [Decimal; 3] = [Decimal::new(0, 64); 3];
        for i in 0..3 {
            let s: &str = darr[i].as_str();
            match Decimal::from_str(s) {
                Err(_) => panic!("can not instantiate decimal"),
                Ok(d) => {
                    xyz[i] = d;
                }
            }
        }
        xyz
    }

    pub fn add(&self, other: &Vec3d) -> Vec3d {
        //
        let m = self.data() + other.data();
        Vec3d::new(m)
    }

    pub fn subtract(&self, other: &Vec3d) -> Vec3d {
        //
        let m = self.data() - other.data();
        Vec3d::new(m)
    }

    pub fn multiply(&self, other: &Vec3d) -> Vec3d {
        //
        let m = self.data().component_mul(&other.data());
        Vec3d::new(m)
    }
    pub fn dot(&self, other: &Vec3d) -> real {
        //
        let m = self.data().dot(&other.data());
        m
    }
    pub fn norm(&self) -> real {
        let m = self.data().norm() as real;
        m
    }
    pub fn to_unit(&self) -> Vec3d {
        let norm = self.data().norm() as real;
        let m = self.scalar_divide(norm);
        m
    }

    pub fn scalar_add(&self, other: real) -> Vec3d {
        //
        let m = self.data().add_scalar(other);
        Vec3d::new(m)
    }

    pub fn scalar_subtract(&self, other: real) -> Vec3d {
        //
        let m = self.data().add_scalar(-1.0 * other);
        Vec3d::new(m)
    }

    pub fn scalar_multiply(&self, other: real) -> Vec3d {
        //
        let m = self.data().scale(other);
        Vec3d::new(m)
    }

    pub fn scalar_divide(&self, other: real) -> Vec3d {
        //
        let m = self.data().unscale(other);
        Vec3d::new(m)
    }
}

impl PartialEq for Vec3d {
    fn eq(&self, other: &Vec3d) -> bool {
        let my_arr = self.to_decimal_array();
        let other_arr = other.to_decimal_array();
        let mut result = true;
        let nb_elements = my_arr.len();
        for i in 0..nb_elements {
            result = result && (my_arr[i] == other_arr[i]);
        }
        result
    }
}
