mod ray;
mod vec3;
mod objects;
mod hittable;
mod camera;
mod utility;

use ray::{Ray, ray};
use vec3::{Vec3, Point};
use objects::{Sphere};
use hittable::{HitResult, HittableList};
use camera::Camera;
use utility::random_float;
use image::{ImageBuffer, Rgb};
use image::imageops::{flip_vertical_in_place};
use std::time::{Instant};

use Vec3 as Color;


#[inline]
fn tonemap_pixel(color: Color, samples: f64) -> [u8; 3] {
    let mut r: f64 = color.x;
    let mut g: f64 = color.y;
    let mut b: f64 = color.z;

    let scale: f64 = 1.0 / samples;

    r *= scale;
    g *= scale;
    b *= scale;

    [
        (255.999 * r.clamp(0.0, 0.999)) as u8,
        (255.999 * g.clamp(0.0, 0.999)) as u8,
        (255.999 * b.clamp(0.0, 0.999)) as u8
    ]
}

#[inline]
fn unit_vector(vec: Vec3) -> Vec3 {
    vec.normalize()
}

fn ray_color(r: Ray, world: &HittableList) -> Color {
    let result: HitResult = world.hit(&r, 0.0, f64::INFINITY);
    if result.hit {
        return 0.5 * (result.hit_record.normal + Vec3::ONE);
    }
    let unit_direction: Vec3 = unit_vector(r.direction());
    let t: f64 = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0) as Color
}

fn main() {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(image_width, image_height);
    let samples: f64 = 100.0;

    //World
    let mut world: HittableList = HittableList::hittable_list();
    world.objects.push(Box::new(Sphere::sphere(0.0, 0.0, -1.0, 0.5)));
    world.objects.push(Box::new(Sphere::sphere(0.0, -100.5, -1.0, 100.0)));

    //Camera
    let camera: Camera = Camera::camera();

    let timer: Instant = Instant::now();

    //Render
    for (x,y,pixel) in img.enumerate_pixels_mut() {
        eprint!("\rScanlines remaining: {}", (image_height - 1) - y);

        let mut pixel_color = Vec3::ZERO;
        for _s in 0..samples as u64 {
            let u: f64 = (x as f64 + random_float()) / (image_width - 1) as f64;
            let v: f64 = (y as f64 + random_float()) / (image_height - 1) as f64;

            let r: Ray = camera.get_ray(u, v);
            pixel_color += ray_color(r, &world);
        }

        *pixel = Rgb(tonemap_pixel(pixel_color, samples));
    }

    println!("\nElapsed time: {}ms", timer.elapsed().as_millis());
    //7739

    flip_vertical_in_place(&mut img);
    img.save("output/test7.png").unwrap()
}