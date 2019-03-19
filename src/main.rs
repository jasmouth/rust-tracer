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
use std::sync::Arc;
use std::thread;

use camera::Camera;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use hitable::hitable_list::HitableList;
use hitable::materials::{Dielectric, Lambertian, Material, Metal};
use hitable::sphere::Sphere;
use ray::Ray;
use vec3::{unit_vector, Vec3};

static NUM_THREADS: i32 = 8;

/// Calculates a final color value for a given Ray
fn get_color(ray: &Ray, world: &HitableList, depth: i32) -> Vec3 {
    let ref mut rec = HitRecord::new();
    if world.hit(ray, 0.0001, FLOAT_MAX, rec) {
        let (scatteredRay, attenuation, didScatter) = match rec.material {
            Some(ref mat) => mat.scatter(ray, rec),
            None => (
                Ray {
                    origin: ray.origin,
                    direction: ray.direction,
                },
                Vec3::new(0.0, 0.0, 0.0),
                false,
            ),
        };
        if depth < 50 && didScatter {
            return attenuation * get_color(&scatteredRay, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = unit_vector(ray.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 0.55, 0.0) + t * Vec3::new(0.2, 0.1, 0.5);
    }
}

fn create_rand_scene(
    mut rng: rand::prelude::ThreadRng,
    range: &rand::distributions::Uniform<f64>,
) -> HitableList {
    let mut sphere_list = vec![Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.2, 0.2, 0.2),
        }),
    }) as Box<Hitable>];

    for a in -11..11 {
        for b in -11..11 {
            let material_choice = range.sample(&mut rng);
            let center = Vec3::new(
                a as f64 + 0.9 * range.sample(&mut rng),
                0.2,
                b as f64 + 0.9 * range.sample(&mut rng),
            );
            if ((center - Vec3::new(4.0, 0.2, 0.0)).length() <= 0.9)
                || ((center - Vec3::new(0.0, 0.2, 1.0)).length() <= 0.9)
                || ((center - Vec3::new(2.0, 0.2, -2.0)).length() <= 0.9)
            {
                continue;
            }
            let material: Box<Material>;
            if material_choice < 0.75 {
                // Matte
                material = Box::new(Lambertian {
                    albedo: Vec3::new(
                        range.sample(&mut rng) * range.sample(&mut rng),
                        range.sample(&mut rng) * range.sample(&mut rng),
                        range.sample(&mut rng) * range.sample(&mut rng),
                    ),
                });
            } else if material_choice < 0.9 {
                // Metal
                material = Box::new(Metal::new(
                    Vec3::new(
                        0.5 * (1.0 + range.sample(&mut rng)),
                        0.5 * (1.0 + range.sample(&mut rng)),
                        0.5 * (1.0 + range.sample(&mut rng)),
                    ),
                    0.5 * range.sample(&mut rng),
                ));
            } else if material_choice < 0.95 {
                // Glass
                material = Box::new(Dielectric::new(1.5));
            } else {
                // Diamond
                material = Box::new(Dielectric::new(2.4));
            }
            sphere_list.push(Box::new(Sphere {
                center,
                radius: 0.2,
                material,
            }));
        }
    }

    sphere_list.push(Box::new(Sphere {
        center: Vec3::new(2.0, 1.0, -2.0),
        radius: 1.0,
        material: Box::new(Dielectric::new(1.5)),
    }));
    sphere_list.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 1.0),
        radius: 1.0,
        material: Box::new(Lambertian {
            albedo: Vec3::new(1.0, 1.0, 1.0),
        }),
    }));
    sphere_list.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal::new(Vec3::new(0.5, 0.5, 0.5), 0.0)),
    }));

    HitableList { list: sphere_list }
}

fn main() {
    let numX = 1200;
    let numY = 800;
    let numSamples = 100;
    let range = Uniform::new_inclusive(0.0, 1.0);
    let mut imgBuff = image::ImageBuffer::new(numX, numY);
    let look_from = Vec3::new(13.0, 2.5, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        numX as f64 / numY as f64,
        0.1,
        10.0,
    );
    let world = Arc::new(create_rand_scene(rand::thread_rng(), &range));

    for y in 0..numY {
        for x in 0..numX {
            let mut child_threads = vec![];
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..NUM_THREADS {
                let _world = Arc::clone(&world);
                child_threads.push(thread::spawn(move || -> Vec3 {
                    let mut _color = Vec3::new(0.0, 0.0, 0.0);
                    let mut rng = rand::thread_rng();
                    for _ in 0..(numSamples / NUM_THREADS) {
                        let u = (x as f64 + range.sample(&mut rng)) / (numX as f64);
                        let v = (y as f64 + range.sample(&mut rng)) / (numY as f64);
                        let ray = camera.create_ray(u, v);
                        _color += get_color(&ray, &_world, 0);
                    }
                    _color
                }));
            }
            for thread in child_threads {
                color += thread.join().unwrap();
            }
            color /= numSamples as f64;
            let pixel = image::Rgb([
                (color.r().sqrt() * 255.99) as u8,
                (color.g().sqrt() * 255.99) as u8,
                (color.b().sqrt() * 255.99) as u8,
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
