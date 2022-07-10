use crate::vec3::{Vec3, Point};
use crate::ray::{Ray};

struct HitRecord {
    p: Point,
    normal: Vec3,
    t: f64,
    front_face: bool
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal}
    }
}

trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point,
    radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = Vec3::dot(&oc, &r.direction());
        let c: f64 =  oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {return false}
        let sqrt_d: f64 = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range.
        let root: f64 = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {return false}
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return true
    }
}