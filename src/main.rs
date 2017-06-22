#![allow(non_snake_case)]

extern crate image;
extern crate nalgebra as na;

pub mod ray;

use std::fs::File;
use std::path::Path;
use na::{Vector3 as Vec3, Unit};
use ray::Ray;

fn hit_sphere(center: Vec3<f64>, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = na::dot(&ray.direction, &ray.direction);
    let b = 2.0 * na::dot(&oc, &ray.direction);
    let c = na::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn get_color(ray: &Ray) -> Vec3<f64> {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = Unit::new_normalize(ray.direction).unwrap();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let numX = 200;
    let numY = 100;
    let mut imgBuff = image::ImageBuffer::new(numX, numY);

    let lowerLeftCorner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for y in 0..numY {
        for x in 0..numX {
            let u = (x as f64) / (numX as f64);
            let v = (y as f64) / (numY as f64);
            let ray = Ray {
                origin: origin,
                direction: lowerLeftCorner + u * horizontal + v * vertical,
            };
            let color = get_color(&ray);
            let pixel = image::Rgb([(color.x * 255.99) as u8,
                                    (color.y * 255.99) as u8,
                                    (color.z * 255.99) as u8]);
            // Invert y coordinate
            imgBuff.put_pixel(x, (numY - 1) - y, pixel);
        }
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    let _ = image::ImageRgb8(imgBuff).save(fout, image::PNG);
}
