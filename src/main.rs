#![allow(non_snake_case)]

extern crate image;
extern crate rand;

pub mod camera;
pub mod hitable;
pub mod ray;
pub mod vec3;

use rand::distributions::{Distribution, Uniform};
use std::f64::MAX as FLOAT_MAX;
use std::fs::File;
use std::path::Path;

use camera::Camera;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use hitable::hitable_list::HitableList;
use hitable::sphere::Sphere;
use ray::Ray;
use vec3::{dot, unit_vector, Vec3};

fn random_point_in_unit_sphere() -> Vec3 {
    let range = Uniform::new_inclusive(0.0, 1.0);
    let mut rng = rand::thread_rng();
    let mut point;
    loop {
        point =
            2.0 * Vec3::new(
                range.sample(&mut rng),
                range.sample(&mut rng),
                range.sample(&mut rng),
            ) - Vec3::new(1.0, 1.0, 1.0);
        if dot(&point, &point) < 1.0 {
            break;
        }
    }

    point
}

fn get_color(ray: &Ray, world: &HitableList) -> Vec3 {
    let ref mut rec = HitRecord::new();
    if world.hit(ray, 0.0001, FLOAT_MAX, rec) {
        let target = rec.hit_point + rec.normal + random_point_in_unit_sphere();
        return 0.5
            * get_color(
                &Ray {
                    origin: rec.hit_point,
                    direction: target - rec.hit_point,
                },
                world,
            );
    } else {
        let unit_direction = unit_vector(ray.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let numX = 200;
    let numY = 100;
    let numSamples = 100;
    let range = Uniform::new_inclusive(0.0, 1.0);
    let mut rng = rand::thread_rng();
    let mut imgBuff = image::ImageBuffer::new(numX, numY);
    let camera = Camera::new();
    let world = HitableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    };

    for y in 0..numY {
        for x in 0..numX {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..numSamples {
                let u = (x as f64 + range.sample(&mut rng)) / (numX as f64);
                let v = (y as f64 + range.sample(&mut rng)) / (numY as f64);
                let ray = camera.create_ray(u, v);
                color += get_color(&ray, &world);
            }
            color /= numSamples as f64;
            let pixel = image::Rgb([
                (color.x().sqrt() * 255.99) as u8,
                (color.y().sqrt() * 255.99) as u8,
                (color.z().sqrt() * 255.99) as u8,
            ]);
            // Invert y coordinate
            imgBuff.put_pixel(x, (numY - 1) - y, pixel);
        }
    }

    let path = &Path::new("output.png");
    match File::create(path) {
        Ok(_) => {
            let _ = image::ImageRgb8(imgBuff).save(path);
        }
        Err(e) => println!("Failed to open file: {:?}", e),
    }
}
