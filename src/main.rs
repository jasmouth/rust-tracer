#![allow(non_snake_case)]

extern crate image;
extern crate nalgebra as na;

pub mod ray;
pub mod hitable;

use na::{Vector3 as Vec3, Unit};
use std::fs::File;
use std::path::Path;
use std::f64::MAX as FLOAT_MAX;

use hitable::sphere::Sphere;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use hitable::hitable_list::HitableList;
use ray::Ray;

fn get_color(ray: &Ray, world: &HitableList) -> Vec3<f64> {
    let ref mut rec = HitRecord::new();
    if world.hit(ray, 0.0, FLOAT_MAX, rec) {
        return 0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
    } else {
        let unit_direction = Unit::new_normalize(ray.direction).unwrap();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let numX = 200;
    let numY = 100;
    let mut imgBuff = image::ImageBuffer::new(numX, numY);

    let lowerLeftCorner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let world = HitableList {
        list: vec![Box::new(Sphere {
                       center: Vec3::new(0.0, 0.0, -1.0),
                       radius: 0.5,
                   }),
                   Box::new(Sphere {
                       center: Vec3::new(0.0, -100.5, -1.0),
                       radius: 100.0,
                   })],
    };

    for y in 0..numY {
        for x in 0..numX {
            let u = (x as f64) / (numX as f64);
            let v = (y as f64) / (numY as f64);
            let ray = Ray {
                origin: origin,
                direction: lowerLeftCorner + u * horizontal + v * vertical,
            };
            let color = get_color(&ray, &world);
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
