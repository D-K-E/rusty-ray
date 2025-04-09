/// color io
use crate::domain::math3d::ray::Ray;
use crate::domain::math3d::sphere::Sphere;
use crate::domain::math3d::vector::Vec3d;

pub fn ray_color_v1(r: &Ray) -> Vec3d {
    let sphere = Sphere::new(Vec3d::from_xyz(0.0, 0.0, -1.0), 0.5);
    let is_hit = sphere.is_hit(r);
    if is_hit {
        Vec3d::from_xyz(1.0, 0.0, 0.0)
    } else {
        let dir = r.direction().to_unit();
        let yval = dir.y();
        let tval = (yval + 1.0) * 0.5;
        let one_min = 1.0 - tval;
        let cval = Vec3d::from_scalar(1.0).scalar_multiply(one_min);
        let oval = Vec3d::from_xyz(0.5, 0.7, 1.0).scalar_multiply(tval);
        cval.add(&oval)
    }
}
