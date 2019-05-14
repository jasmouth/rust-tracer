extern crate image;
extern crate indicatif;
extern crate num_cpus;
extern crate rand;
extern crate regex;

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
use regex::Regex;
use std::f64::MAX as FLOAT_MAX;
use std::fs::File;
use std::io::prelude::*;
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
use hitable::polygon::{Polygon, PolygonMesh};
use hitable::rectangles::{AxisAlignedBlock, XYRect, XZRect, YZRect};
use hitable::sphere::Sphere;
use hitable::transformations::{RotateY, Translate};
use hitable::volumes::{ConstantMedium, VariableMedium};
use material::material::Material;
use material::materials::{Dielectric, DiffuseLight, Glossy, Lambertian, Metal};
use ray::Ray;
use texture::textures::{CheckerTexture, ConstantTexture, NoiseTexture};
use vec3::Vec3;

static MAX_DEPTH: i32 = 5;

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
    let light = DiffuseLight::new(Box::new(ConstantTexture::new(Vec3::new(2.0, 2.0, 2.0))));
    let left_wall = Box::new(YZRect {
        material: Box::new(green),
        y_0: 0.0,
        y_1: 20.0,
        z_0: -10.0,
        z_1: 10.0,
        k: -10.0,
    });
    let right_wall = Box::new(YZRect {
        material: Box::new(red.clone()),
        y_0: 0.0,
        y_1: 20.0,
        z_0: -10.0,
        z_1: 10.0,
        k: 10.0,
    });
    let back_wall = Box::new(XYRect {
        material: Box::new(white.clone()),
        x_0: -10.0,
        x_1: 10.0,
        y_0: 0.0,
        y_1: 20.0,
        k: 10.0,
    });
    let front_wall = Box::new(XYRect {
        material: Box::new(white.clone()),
        x_0: -10.0,
        x_1: 10.0,
        y_0: 0.0,
        y_1: 20.0,
        k: -10.0,
    });
    let _front_light = Box::new(XYRect {
        material: Box::new(DiffuseLight::new(Box::new(ConstantTexture::new(
            Vec3::new(2.0, 2.0, 2.0),
        )))),
        x_0: 0.0,
        x_1: 555.0,
        y_0: 0.0,
        y_1: 555.0,
        k: -605.0,
    });
    let floor = Box::new(XZRect {
        material: Box::new(white.clone()),
        x_0: -10.0,
        x_1: 10.0,
        z_0: -10.0,
        z_1: 10.0,
        k: 0.0,
    });
    let ceiling = Box::new(XZRect {
        material: Box::new(white.clone()),
        x_0: -10.0,
        x_1: 10.0,
        z_0: -10.0,
        z_1: 10.0,
        k: 20.0,
    });
    let ceiling_light = Box::new(XZRect {
        material: Box::new(light.clone()),
        x_0: -7.5,
        x_1: 7.5,
        z_0: -7.5,
        z_1: 7.5,
        k: 20.0,
    });

    let pedestal = Box::new(AxisAlignedBlock::new(
        Vec3::new(-2.0, 0.0, -3.0),
        Vec3::new(2.0, 7.95, 1.0),
        Box::new(Lambertian {
            albedo: Box::new(ConstantTexture::new(Vec3::new(0.396, 0.263, 0.129))),
        }),
    ));
    let _ball_light = Box::new(Sphere {
        material: Box::new(DiffuseLight::new(Box::new(ConstantTexture::new(
            Vec3::new(1.75, 1.75, 1.75),
        )))),
        center: Vec3::new(278.0, 258.0, 278.0),
        radius: 35.0,
    });

    let teapot = parse_obj_file(
        String::from("object-files/teapot.obj"),
        Box::new(Dielectric::new(1.5)),
    );
    let teapot_subsurface = Box::new(ConstantMedium::new(
        teapot.clone(),
        10.0,
        Box::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 0.941))),
    ));
    let list: Vec<Box<Hitable>> = vec![
        left_wall,
        Box::new(FlipNormals::new(right_wall)),
        Box::new(FlipNormals::new(back_wall)),
        front_wall,
        floor,
        Box::new(FlipNormals::new(ceiling)),
        // _ball_light,
        ceiling_light,
        Box::new(Translate::new(teapot, Vec3::new(0.0, 8.0, -1.5))),
        Box::new(Translate::new(teapot_subsurface, Vec3::new(0.0, 8.0, -1.5))),
        pedestal,
    ];

    BvhNode::new(&mut HitableList { list }, 0.0, 0.0)
}

fn create_debug_scene() -> BvhNode {
    #![allow(dead_code)]
    // Shared material defs
    let light_blue = Lambertian {
        albedo: Box::new(ConstantTexture::new(Vec3::new(0.5304, 0.6152, 0.7688))),
    };

    // Wall defs
    let back_wall = Box::new(FlipNormals::new(Box::new(XYRect {
        material: Box::new(light_blue.clone()),
        x_0: -700.0,
        x_1: 700.0,
        y_0: 0.0,
        y_1: 800.0,
        k: 1000.0,
    })));
    let front_wall = Box::new(XYRect {
        material: Box::new(light_blue.clone()),
        x_0: -700.0,
        x_1: 700.0,
        y_0: 0.0,
        y_1: 800.0,
        k: -700.0,
    });
    let left_wall = Box::new(FlipNormals::new(Box::new(YZRect {
        material: Box::new(light_blue.clone()),
        y_0: 0.0,
        y_1: 800.0,
        z_0: -700.0,
        z_1: 1000.0,
        k: 700.0,
    })));
    let right_wall = Box::new(YZRect {
        material: Box::new(light_blue.clone()),
        y_0: 0.0,
        y_1: 800.0,
        z_0: -700.0,
        z_1: 1000.0,
        k: -700.0,
    });
    let floor = Box::new(XZRect {
        material: Box::new(light_blue.clone()),
        x_0: -700.0,
        x_1: 700.0,
        z_0: -700.0,
        z_1: 1000.0,
        k: 0.0,
    });
    let ceiling = Box::new(FlipNormals::new(Box::new(XZRect {
        material: Box::new(light_blue.clone()),
        x_0: -700.0,
        x_1: 700.0,
        z_0: -700.0,
        z_1: 1000.0,
        k: 800.0,
    })));
    let ceiling_light = Box::new(XZRect {
        material: Box::new(DiffuseLight::new(Box::new(ConstantTexture::new(
            Vec3::new(7.5, 7.5, 7.5),
        )))),
        x_0: 100.0,
        x_1: 700.0,
        z_0: 200.0,
        z_1: 500.0,
        k: 800.0,
    });

    let teapot = parse_obj_file(
        String::from("object-files/teapot.obj"),
        Box::new(Glossy::new(
            Box::new(ConstantTexture::new(Vec3::new(0.8, 0.8, 0.7528))),
            0.5,
        )),
    );

    let list: Vec<Box<Hitable>> = vec![
        ceiling,
        ceiling_light,
        left_wall,
        back_wall,
        front_wall,
        right_wall,
        floor,
        teapot,
    ];
    BvhNode::new(&mut HitableList { list }, 0.0, 1.0)
}

fn create_final_scene() -> BvhNode {
    #![allow(dead_code)]
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
            albedo: Box::new(NoiseTexture::new(0.05, 8)),
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
        0.5,
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9))),
    ));
    let mist_boundary = Box::new(Sphere {
        material: Box::new(Dielectric::new(1.5)), // arbitrary material
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 5000.0,
    });
    let mist = Box::new(VariableMedium::new(
        mist_boundary,
        0.0002,
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

/// Recreates the "wada2" scene from smallpt (http://www.kevinbeason.com/smallpt/)
///
/// Note: The MAX_DEPTH variable needs to be increased (~50?) for this to properly render
fn wada() -> BvhNode {
    let radius = 120.0;
    let theta = 30.0 * std::f64::consts::PI / 180.0;
    let distance = radius / theta.cos();
    let color = Vec3::new(0.275, 0.612, 0.949);

    let list: Vec<Box<Hitable>> = vec![
        Box::new(FlipNormals::new(Box::new(Sphere {
            radius: 2.0 * 2.0 * radius * 2.0 * (2_f64 / 3_f64).sqrt()
                - radius * 2.0 * (2_f64 / 3_f64).sqrt() / 3.0,
            center: Vec3::new(50.0, 28.0, 62.0)
                + Vec3::new(0.0, 0.0, -radius * 2.0 * (2_f64 / 3_f64).sqrt() / 3.0),
            material: Box::new(Metal::new(
                Box::new(ConstantTexture::new(Vec3::new(0.5, 0.5, 0.5))),
                0.0,
            )),
        }))),
        Box::new(Sphere {
            radius,
            center: Vec3::new(50.0, 28.0, 62.0)
                + Vec3::new(0.0, 0.0, -1.0) * radius * 2.0 * (2_f64 / 3_f64).sqrt(),
            material: Box::new(Metal::new(
                Box::new(ConstantTexture::new(Vec3::new(0.996, 0.996, 0.996))),
                0.0,
            )),
        }),
        Box::new(Sphere {
            radius,
            center: Vec3::new(50.0, 28.0, 62.0) + Vec3::new(0.0, -1.0, 0.0) * distance,
            material: Box::new(Metal::new_emitting(
                Box::new(ConstantTexture::new(Vec3::new(0.996, 0.996, 0.996))),
                Box::new(ConstantTexture::new(color * 6e-2)),
                0.0,
            )),
        }),
        Box::new(Sphere {
            radius,
            center: Vec3::new(50.0, 28.0, 62.0)
                + Vec3::new(-(theta.cos()), theta.sin(), 0.0) * distance,
            material: Box::new(Metal::new_emitting(
                Box::new(ConstantTexture::new(Vec3::new(0.996, 0.996, 0.996))),
                Box::new(ConstantTexture::new(color * 6e-2)),
                0.0,
            )),
        }),
        Box::new(Sphere {
            radius,
            center: Vec3::new(50.0, 28.0, 62.0)
                + Vec3::new(theta.cos(), theta.sin(), 0.0) * distance,
            material: Box::new(Metal::new_emitting(
                Box::new(ConstantTexture::new(Vec3::new(0.996, 0.996, 0.996))),
                Box::new(ConstantTexture::new(color * 6e-2)),
                0.0,
            )),
        }),
    ];
    BvhNode::new(&mut HitableList { list }, 0.0, 0.0)
}

/// Provides primitive ability for parsing an OBJ file.
fn parse_obj_file(file_name: String, material: Box<Material>) -> Box<Hitable> {
    let path = &Path::new(&file_name);
    match File::open(path) {
        Ok(mut file) => {
            let mut data = String::new();
            file.read_to_string(&mut data)
                .expect("Could not read data from file!");
            let mut vertices: Vec<Vec3> = vec![];
            let mut vertex_normals: Vec<Vec3> = vec![];
            let mut poly_list: Vec<Box<Hitable>> = vec![];
            let reg = Regex::new(r"//(.+)").unwrap();
            data.lines().for_each(|line: &str| {
                if line.starts_with("v ") {
                    let vec = line
                        .split_whitespace()
                        .into_iter()
                        .filter_map(|v: &str| v.parse::<f64>().ok())
                        .collect::<Vec<f64>>();
                    let vertex = Vec3::from_vec(vec);
                    vertices.push(vertex);
                } else if line.starts_with("vn ") {
                    let normal = line
                        .split_whitespace()
                        .into_iter()
                        .filter_map(|v: &str| v.parse::<f64>().ok())
                        .collect::<Vec<f64>>();
                    let vert_normal = Vec3::from_vec(normal);
                    vertex_normals.push(vert_normal);
                } else if line.starts_with("f ") {
                    let points = line
                        .split_whitespace()
                        .into_iter()
                        .filter_map(|v: &str| {
                            if v.contains("//") {
                                reg.replace(v, "").parse::<usize>().ok()
                            } else {
                                v.parse::<usize>().ok()
                            }
                        })
                        .map(|i| vertices[i - 1])
                        .collect::<Vec<Vec3>>();
                    let normals = line
                        .split_whitespace()
                        .into_iter()
                        .filter_map(|v: &str| {
                            if v.contains("//") {
                                reg.captures(v)
                                    .unwrap()
                                    .get(1)
                                    .unwrap()
                                    .as_str()
                                    .parse::<usize>()
                                    .ok()
                            } else {
                                None
                            }
                        })
                        .map(|i| vertex_normals[i - 1])
                        .collect::<Vec<Vec3>>();
                    // If a face is defined by more than 3 points, break it into a triangle fan
                    for n in 0..=(points.len() - 3) {
                        let mut face = Box::new(Polygon::new(
                            vec![points[0], points[n + 1], points[n + 2]],
                            material.clone(),
                        ));
                        if !normals.is_empty() {
                            face.vertex_normals =
                                Some(vec![normals[0], normals[n + 1], normals[n + 2]]);
                        }
                        poly_list.push(face);
                    }
                }
            });
            Box::new(PolygonMesh::new(poly_list))
        }
        Err(e) => {
            println!("Failed to open file: {:?}", e);
            Box::new(PolygonMesh::new(vec![]))
        }
    }
}

fn main() {
    let num_threads: usize = num_cpus::get() - 1;
    // let num_x = 350;
    // let num_y = 350;
    // let num_samples_per_thread = 572;
    let num_x = 1024;
    let num_y = 768;
    let num_samples_per_thread = 1;
    let num_samples = num_threads * num_samples_per_thread;
    let range = Uniform::new(0.0, 1.0);
    let mut img_buff = image::ImageBuffer::new(num_x, num_y);
    let look_from = Vec3::new(50.0, 52.0, 295.6);
    let look_in = Vec3::new(0.0, -0.042612, -1.0);
    let camera = Camera::new(
        look_from,                   // Camera origin
        look_in,                     // Camera view direction
        Vec3::new(0.0, 1.0, 0.0),    // Camera "up" direction
        30.0,                        // Vertical FOV
        num_x as f64 / num_y as f64, // Aspect ratio
        0.0,                         // Aperture
        10.0,                        // Focus Distance
        0.0,                         // Shutter open time
        1.0,                         // Shutter close time
    );
    // let world = Arc::new(create_rand_scene(rand::thread_rng(), &range));
    // let world = Arc::new(create_cornell_box());
    // let world = Arc::new(create_debug_scene());
    // let world = Arc::new(create_final_scene());
    let world = Arc::new(wada());

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
