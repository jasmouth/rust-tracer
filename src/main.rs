#![allow(non_snake_case)]

extern crate image;
extern crate nalgebra as na;

pub mod ray;
use ray::Ray;
use std::fs::File;
use std::path::Path;
use na::{Vector3, Unit};

fn get_color(r: Ray) -> Vector3<f64> {
    let unit_direction = Unit::new_normalize(r.direction).unwrap();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn main() {
    let numX = 200;
    let numY = 100;
    let mut imgBuff = image::ImageBuffer::new(numX, numY);

    let lowerLeftCorner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    for y in (0..numY).rev() {
        for x in 0..numX {
            let u = (x as f64) / (numX as f64);
            let v = (y as f64) / (numY as f64);
            let ray = Ray {
                origin: origin,
                direction: lowerLeftCorner + u * horizontal + v * vertical,
            };
            let color = get_color(ray);
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
