use crate::vec3::{Vec3, Point};

pub struct Ray {
    origin: Point,
    direction: Vec3
}

impl Ray {
    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}

pub fn ray(origin: Point, direction: Vec3) -> Ray {
    Ray{origin: origin, direction: direction}
}