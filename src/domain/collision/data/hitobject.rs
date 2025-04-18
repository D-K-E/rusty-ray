//! hit object
use crate::domain::collision::data::sphere::Sphere;
use crate::domain::math3d::constant::real;
use crate::domain::math3d::vector::Vec3d;

#[derive(Clone, PartialEq)]
pub enum ObjectKind {
    Sphere,
}

#[derive(Clone, PartialEq)]
pub struct HitObject {
    kind: ObjectKind,
    position: Vec3d,
    radius: real,
}

impl HitObject {
    pub fn from_sphere(s: &Sphere) -> HitObject {
        HitObject {
            position: s.center().clone(),
            radius: s.radius(),
            kind: ObjectKind::Sphere,
        }
    }
    pub fn to_sphere(&self) -> Sphere {
        Sphere::new(self.position().clone(), self.radius())
    }
    pub fn position(&self) -> &Vec3d {
        &self.position
    }

    pub fn radius(&self) -> real {
        self.radius.clone()
    }

    pub fn kind(&self) -> ObjectKind {
        self.kind.clone()
    }
}
