mod ray;
mod vec3;

use ray::{Ray, ray};
use vec3::{Vec3, Point};
use image::{ImageBuffer, Rgb};
use image::imageops::{flip_vertical_in_place};

use Vec3 as Color;

#[inline]
fn tonemap_pixel(color: Color) -> [u8; 3] {
    [(255.999 * color.x) as u8, (255.999 * color.y) as u8, (255.999 * color.z) as u8]
}

#[inline]
fn unit_vector(vec: Vec3) -> Vec3 {
    vec.normalize()
}

fn hit_sphere(center: Point, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin() - center;
    let a: f64 = Vec3::dot(&r.direction(), &r.direction());
    let b: f64 = 2.0 * Vec3::dot(&oc, &r.direction());
    let c: f64 =  Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant: f64 = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: Ray) -> Color {
    let mut t: f64 = hit_sphere(Vec3::new(0.0, 0.0, -1.0) as Point, 0.5, &r);
    if t > 0.0 {
        let N: Vec3 = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * (N + 1.0);
    }
    let unit_direction: Vec3 = unit_vector(r.direction());
    t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0) as Color
}

fn main() {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(image_width, image_height);

    //Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Point = Vec3::ZERO as Point;
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Point = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    for (x,y,pixel) in img.enumerate_pixels_mut() {
        eprint!("\rScanlines remaining: {}", (image_height - 1) - y);

        let u: f64 = x as f64 / (image_width - 1) as f64;
        let v: f64 = y as f64 / (image_height - 1) as f64;

        let r: Ray = ray(origin, lower_left_corner + u * horizontal + v * vertical - origin);
        let pixel_color: Color = ray_color(r);

        *pixel = Rgb(tonemap_pixel(pixel_color));
    }

    flip_vertical_in_place(&mut img);
    img.save("output/test4.png").unwrap()
}