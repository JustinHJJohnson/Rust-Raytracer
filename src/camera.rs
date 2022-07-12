use crate::ray::{Ray, ray};
use crate::vec3::{Vec3, Point};

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,

    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        ray(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }

    pub fn camera() -> Camera {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;
        let origin: Vec3 = Vec3::ZERO;
        let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
        
        Camera { 
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length) 
        }
    }
}