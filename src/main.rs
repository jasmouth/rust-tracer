extern crate image;
extern crate indicatif;
extern crate num_cpus;
extern crate rand;

pub mod bounding_boxes;
pub mod camera;
pub mod hitable;
pub mod material;
pub mod ray;
pub mod texture;
pub mod vec3;

use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::f64::MAX as FLOAT_MAX;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::thread;

use camera::Camera;
use hitable::bvh_node::BvhNode;
use hitable::flip_normals::FlipNormals;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use hitable::hitable_list::HitableList;
use hitable::moving_sphere::MovingSphere;
use hitable::rectangles::{AxisAlignedBlock, XYRect, XZRect, YZRect};
use hitable::sphere::Sphere;
use hitable::transformations::{RotateY, Translate};
use hitable::volumes::{ConstantMedium, VariableMedium};
use material::materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use ray::Ray;
use texture::textures::{CheckerTexture, ConstantTexture, NoiseTexture};
use vec3::Vec3;

static MAX_DEPTH: i32 = 50;

/// Calculates a final color value for a given Ray
fn get_color(ray: &Ray, world: &BvhNode, depth: i32) -> Vec3 {
    let ref mut rec = HitRecord::new();
    if world.hit(ray, 0.00001, FLOAT_MAX, rec) {
        let ((scattered_ray, attenuation, did_scatter), emitted_light) = match rec.material {
            Some(ref mat) => (
                mat.scatter(ray, rec),
                mat.emit(rec.u, rec.v, &rec.hit_point),
            ),
            None => (
                (
                    Ray::new(ray.origin, ray.direction, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    false,
                ),
                Vec3::new(0.0, 0.0, 0.0),
            ),
        };
        if depth < MAX_DEPTH && did_scatter {
            return emitted_light + attenuation * get_color(&scattered_ray, world, depth + 1);
        } else {
            return emitted_light;
        }
    } else {
        return Vec3::new(0.0, 0.0, 0.0);
    }
}

fn create_rand_scene(
    mut rng: rand::prelude::ThreadRng,
    range: &rand::distributions::Uniform<f64>,
) -> BvhNode {
    #![allow(dead_code)]
    let mut sphere_list = vec![Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Box::new(CheckerTexture::new(
                Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
                Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
            )),
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
            let sphere: Box<Hitable> = {
                if material_choice < 0.75 {
                    // Matte
                    Box::new(MovingSphere {
                        start_center: center,
                        end_center: center + Vec3::new(0.0, 0.5 * range.sample(&mut rng), 0.0),
                        start_time: 0.0,
                        end_time: 1.0,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Box::new(ConstantTexture::new(Vec3::new(
                                range.sample(&mut rng) * range.sample(&mut rng),
                                range.sample(&mut rng) * range.sample(&mut rng),
                                range.sample(&mut rng) * range.sample(&mut rng),
                            ))),
                        }),
                    })
                } else if material_choice < 0.9 {
                    // Metal
                    Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal::new(
                            Box::new(ConstantTexture::new(Vec3::new(
                                0.5 * (1.0 + range.sample(&mut rng)),
                                0.5 * (1.0 + range.sample(&mut rng)),
                                0.5 * (1.0 + range.sample(&mut rng)),
                            ))),
                            0.5 * range.sample(&mut rng),
                        )),
                    })
                } else if material_choice < 0.95 {
                    // Glass
                    Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric::new(1.5)),
                    })
                } else {
                    // Diamond
                    Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric::new(2.4)),
                    })
                }
            };
            sphere_list.push(sphere);
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
            albedo: Box::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
        }),
    }));
    sphere_list.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal::new(
            Box::new(ConstantTexture::new(Vec3::new(0.5, 0.5, 0.5))),
            0.0,
        )),
    }));

    let ref mut list = HitableList { list: sphere_list };
    BvhNode::new(list, 0.0, 1.0)
}

fn create_cornell_box() -> BvhNode {
    #![allow(dead_code)]
    let red = Lambertian {
        albedo: Box::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05))),
    };
    let white = Lambertian {
        albedo: Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))),
    };
    let green = Lambertian {
        albedo: Box::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15))),
    };
    let light = DiffuseLight::new(Box::new(ConstantTexture::new(Vec3::new(5.0, 5.0, 5.0))));
    let left_wall = Box::new(YZRect {
        material: Box::new(green),
        y_0: 0.0,
        y_1: 555.0,
        z_0: 0.0,
        z_1: 555.0,
        k: 555.0,
    });
    let right_wall = Box::new(YZRect {
        material: Box::new(red),
        y_0: 0.0,
        y_1: 555.0,
        z_0: 0.0,
        z_1: 555.0,
        k: 0.0,
    });
    let back_wall = Box::new(XYRect {
        material: Box::new(white.clone()),
        x_0: 0.0,
        x_1: 555.0,
        y_0: 0.0,
        y_1: 555.0,
        k: 555.0,
    });
    let floor = Box::new(XZRect {
        material: Box::new(white.clone()),
        x_0: 0.0,
        x_1: 555.0,
        z_0: 0.0,
        z_1: 555.0,
        k: 0.0,
    });
    let ceiling = Box::new(XZRect {
        material: Box::new(white.clone()),
        x_0: 0.0,
        x_1: 555.0,
        z_0: 0.0,
        z_1: 555.0,
        k: 555.0,
    });
    let ceiling_light = Box::new(XZRect {
        material: Box::new(light),
        x_0: 112.0,
        x_1: 443.0,
        z_0: 127.0,
        z_1: 428.0,
        k: 554.0,
    });

    let box_1 = Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(AxisAlignedBlock::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                Box::new(white.clone()),
            )),
            15.0,
        )),
        Vec3::new(265.0, 0.0, 295.0),
    ));
    let box_2 = Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(AxisAlignedBlock::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                Box::new(white.clone()),
            )),
            -18.0,
        )),
        Vec3::new(130.0, 0.0, 65.0),
    ));

    let list: Vec<Box<Hitable>> = vec![
        Box::new(FlipNormals::new(left_wall)),
        right_wall,
        Box::new(FlipNormals::new(back_wall)),
        floor,
        Box::new(FlipNormals::new(ceiling)),
        ceiling_light,
        Box::new(ConstantMedium::new(
            box_1,
            0.01,
            Box::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
        )),
        Box::new(ConstantMedium::new(
            box_2,
            0.01,
            Box::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
        )),
    ];

    BvhNode::new(&mut HitableList { list }, 0.0, 0.0)
}

fn create_debug_scene() -> BvhNode {
    #![allow(dead_code)]
    let red = Lambertian {
        albedo: Box::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05))),
    };
    let white = Lambertian {
        albedo: Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))),
    };
    let light = DiffuseLight::new(Box::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0))));
    let back_wall = Box::new(XYRect {
        material: Box::new(white.clone()),
        x_0: 0.0,
        x_1: 555.0,
        y_0: 0.0,
        y_1: 250.0,
        k: 555.0,
    });
    let ceiling_light = Box::new(XZRect {
        material: Box::new(light),
        x_0: 12.0,
        x_1: 543.0,
        z_0: 27.0,
        z_1: 528.0,
        k: 554.0,
    });
    let floor = Box::new(XZRect {
        material: Box::new(red),
        x_0: 0.0,
        x_1: 555.0,
        z_0: 0.0,
        z_1: 555.0,
        k: 0.0,
    });
    let ball_r = Box::new(Sphere {
        center: Vec3::new(176.5, 125.0, 250.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Box::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
        }),
    });
    let ball_l = Box::new(Sphere {
        center: Vec3::new(378.5, 125.0, 250.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Box::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
        }),
    });
    let list: Vec<Box<Hitable>> = vec![
        ceiling_light,
        Box::new(FlipNormals::new(back_wall)),
        Box::new(ConstantMedium::new(
            ball_r,
            0.01,
            Box::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
        )),
        Box::new(VariableMedium::new(
            ball_l,
            0.5,
            Box::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
        )),
        floor,
    ];
    BvhNode::new(&mut HitableList { list }, 0.0, 0.0)
}

fn create_final_scene() -> BvhNode {
    let mut rng = rand::thread_rng();

    // Ground definition
    let num_boxes = 20;
    let mut box_list: Vec<Box<Hitable>> = vec![];
    let ground = Lambertian {
        albedo: Box::new(ConstantTexture::new(Vec3::new(0.48, 0.83, 0.53))),
    };
    for i in 0..num_boxes {
        for j in 0..num_boxes {
            let width = 100.0;
            let (x_0, y_0, z_0) = (-1000.0 + i as f64 * width, 0.0, -1000.0 + j as f64 * width);
            let (x_1, y_1, z_1) = (x_0 + width, 100.0 * (0.01 + rng.gen::<f64>()), z_0 + width);
            box_list.push(Box::new(AxisAlignedBlock::new(
                Vec3::new(x_0, y_0, z_0),
                Vec3::new(x_1, y_1, z_1),
                Box::new(ground.clone()),
            )));
        }
    }

    // Light definition
    let light = DiffuseLight::new(Box::new(ConstantTexture::new(Vec3::new(7.0, 7.0, 7.0))));
    let ceiling_light = Box::new(XZRect {
        material: Box::new(light),
        x_0: 123.0,
        x_1: 423.0,
        z_0: 147.0,
        z_1: 412.0,
        k: 554.0,
    });

    // Sphere definitions
    let fly_ball = Box::new(MovingSphere {
        material: Box::new(Lambertian {
            albedo: Box::new(ConstantTexture::new(Vec3::new(0.7, 0.3, 0.1))),
        }),
        start_center: Vec3::new(400.0, 400.0, 200.0),
        end_center: Vec3::new(430.0, 400.0, 200.0),
        start_time: 0.0,
        end_time: 1.0,
        radius: 50.0,
    });
    let glass_ball = Box::new(Sphere {
        material: Box::new(Dielectric::new(1.5)),
        center: Vec3::new(260.0, 150.0, 45.0),
        radius: 50.0,
    });
    let metal_ball = Box::new(Sphere {
        material: Box::new(Metal::new(
            Box::new(ConstantTexture::new(Vec3::new(0.8, 0.8, 0.9))),
            10.0,
        )),
        center: Vec3::new(0.0, 150.0, 145.0),
        radius: 50.0,
    });
    let marble_ball = Box::new(Sphere {
        material: Box::new(Lambertian {
            albedo: Box::new(NoiseTexture::new(0.05, 7)),
        }),
        center: Vec3::new(220.0, 280.0, 300.0),
        radius: 80.0,
    });

    // Volume definitions
    let subsurface_boundary = Box::new(Sphere {
        material: Box::new(Dielectric::new(1.5)),
        center: Vec3::new(360.0, 150.0, 145.0),
        radius: 70.0,
    });
    let subsurface_volume = Box::new(ConstantMedium::new(
        subsurface_boundary.clone(),
        0.2,
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9))),
    ));
    let mist_boundary = Box::new(Sphere {
        material: Box::new(Dielectric::new(1.5)), // arbitrary material
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 5000.0,
    });
    let mist = Box::new(ConstantMedium::new(
        mist_boundary,
        0.00005,
        Box::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    ));

    // Sphere-cube definition
    let white = Lambertian {
        albedo: Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))),
    };
    let sphere_cube = (0..1000)
        .map(|_| {
            Box::new(Sphere {
                material: Box::new(white.clone()),
                center: Vec3::new(
                    165.0 * rng.gen::<f64>(),
                    165.0 * rng.gen::<f64>(),
                    165.0 * rng.gen::<f64>(),
                ),
                radius: 10.0,
            }) as Box<Hitable>
        })
        .collect::<Vec<Box<Hitable>>>();
    let sphere_cube = Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(BvhNode::new(
                &mut HitableList { list: sphere_cube },
                0.0,
                1.0,
            )),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    let list: Vec<Box<Hitable>> = vec![
        ceiling_light,
        fly_ball,
        glass_ball,
        metal_ball,
        marble_ball,
        sphere_cube,
        // Note that the combination of the dielectric sphere and the
        // constant volume results in an emulation of a subsurface material.
        subsurface_boundary,
        subsurface_volume,
        mist,
        Box::new(BvhNode::new(&mut HitableList { list: box_list }, 0.0, 1.0)),
    ];
    BvhNode::new(&mut HitableList { list }, 0.0, 1.0)
}

fn main() {
    let num_threads: usize = num_cpus::get();
    let num_x = 800;
    let num_y = 800;
    let num_samples_per_thread = 1250;
    let num_samples = num_threads * num_samples_per_thread;
    let range = Uniform::new(0.0, 1.0);
    let mut img_buff = image::ImageBuffer::new(num_x, num_y);
    let look_from = Vec3::new(478.0, 278.0, -600.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),    // Camera "up" direction
        40.0,                        // Vertical FOV
        num_x as f64 / num_y as f64, // Aspect ratio
        0.0,                         // Aperture
        10.0,                        // Focus Distance
        0.0,                         // Shutter open time
        1.0,                         // Shutter close time
    );
    // let world = Arc::new(create_rand_scene(rand::thread_rng(), &range));
    // let world = Arc::new(create_cornell_box());
    // let world = Arc::new(create_debug_scene());
    let world = Arc::new(create_final_scene());

    let progress_bar = ProgressBar::new((num_x * num_y) as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{percent}%] {bar:40} [{elapsed_precise} | ~{eta} remaining]")
            .progress_chars("=>-"),
    );

    progress_bar.println(format!(
        "Beginning scene tracing using {} CPU cores.",
        num_threads
    ));
    for y in 0..num_y {
        for x in 0..num_x {
            let mut child_threads = vec![];
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..num_threads {
                let _world = Arc::clone(&world);
                child_threads.push(thread::spawn(move || -> Vec3 {
                    let mut _color = Vec3::new(0.0, 0.0, 0.0);
                    let mut rng = rand::thread_rng();
                    for _ in 0..(num_samples / num_threads) {
                        let u = (x as f64 + range.sample(&mut rng)) / (num_x as f64);
                        let v = (y as f64 + range.sample(&mut rng)) / (num_y as f64);
                        let ray = camera.create_ray(u, v);
                        _color += get_color(&ray, &_world, 0);
                    }
                    _color
                }));
            }
            for thread in child_threads {
                color += thread.join().unwrap();
            }
            color /= num_samples as f64;
            let r = (color.r().min(1.0).sqrt() * 255.99) as u8;
            let g = (color.g().min(1.0).sqrt() * 255.99) as u8;
            let b = (color.b().min(1.0).sqrt() * 255.99) as u8;
            let pixel = image::Rgb([r, g, b]);
            // Invert y coordinate
            img_buff.put_pixel(x, (num_y - 1) - y, pixel);
        }
        progress_bar.inc(num_x as u64);
    }
    progress_bar.println("Finished scene tracing.");
    progress_bar.finish();

    let path = &Path::new("output.png");
    match File::create(path) {
        Ok(_) => {
            let _ = image::ImageRgb8(img_buff).save(path);
        }
        Err(e) => println!("Failed to open file: {:?}", e),
    }
}
