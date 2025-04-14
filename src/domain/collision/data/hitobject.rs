//! hit object
use std::marker::Send;
use crate::domain::math3d::constant::real;
use crate::domain::math3d::ray::Ray;
use crate::domain::collision::data::hitrecord::HitRecord;
use crate::domain::collision::data::hitinput::HitInput;
use crate::domain::math3d::vector::Vec3d;
use crate::domain::math3d::sphere::Sphere;
use crate::domain::collision::domain::hittable::Hittable;

enum ObjectKind {
    Sphere
}

#[derive(Clone, Send)]
pub struct HitObject{
    kind: ObjectKind,
    position: Vec3d
    radius: real
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

impl Hittable for HitObject {
    fn is_hit(
        &self,
        r: &Ray,
        min_distance: &real,
        max_distance: &real,
        hit_rec: HitRecord,
    ) -> (HitRecord, bool) {
        if self.kind() == ObjectKind::Sphere {
            let s: Sphere = self.to_sphere();
            let tpl = s.is_hit(r, min_distance, max_distance, hit_rec);
            return tpl;
        }else {
            panic!("unexpected kind");
        }
    }
}
