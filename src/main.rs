#![allow(non_snake_case)]

extern crate image;

use std::fs::File;
use std::path::Path;

fn main() {
    let numX = 200;
    let numY = 100;
    let mut imgBuff = image::ImageBuffer::new(numX, numY);

    for (x, y, pixel) in imgBuff.enumerate_pixels_mut() {
        let r = (x as f32) / (numX as f32);
        let g = (y as f32) / (numY as f32);
        let b = 0.2;
        *pixel = image::Rgb([(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]);
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    let _ = image::ImageRgb8(imgBuff).save(fout, image::PNG);
}
