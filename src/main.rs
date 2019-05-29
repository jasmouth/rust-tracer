extern crate image;
extern crate indicatif;
extern crate num_cpus;
extern crate rand;
extern crate tobj;

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
use std::collections::HashMap;
use std::f64::MAX as FLOAT_MAX;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
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
use texture::texture::Texture;
use texture::textures::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture};
use vec3::Vec3;

static MAX_DEPTH: i32 = 10;

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
        // return Vec3::new(1.0, 1.0, 1.0);
        return Vec3::new(0.0, 0.0, 0.0);
    }
}

fn create_rand_scene(
    mut rng: rand::prelude::ThreadRng,
    range: &rand::distributions::Uniform<f64>,
) -> BvhNode {
    #![allow(dead_code)]
    let mut sphere_list = vec![Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(CheckerTexture::new(
                Arc::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
                Arc::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
            )),
        }),
    }) as Arc<Hitable>];

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
            let sphere: Arc<Hitable> = {
                if material_choice < 0.75 {
                    // Matte
                    Arc::new(MovingSphere {
                        start_center: center,
                        end_center: center + Vec3::new(0.0, 0.5 * range.sample(&mut rng), 0.0),
                        start_time: 0.0,
                        end_time: 1.0,
                        radius: 0.2,
                        material: Arc::new(Lambertian {
                            albedo: Arc::new(ConstantTexture::new(Vec3::new(
                                range.sample(&mut rng) * range.sample(&mut rng),
                                range.sample(&mut rng) * range.sample(&mut rng),
                                range.sample(&mut rng) * range.sample(&mut rng),
                            ))),
                        }),
                    })
                } else if material_choice < 0.9 {
                    // Metal
                    Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal::new(
                            Arc::new(ConstantTexture::new(Vec3::new(
                                0.5 * (1.0 + range.sample(&mut rng)),
                                0.5 * (1.0 + range.sample(&mut rng)),
                                0.5 * (1.0 + range.sample(&mut rng)),
                            ))),
                            0.5 * range.sample(&mut rng),
                        )),
                    })
                } else if material_choice < 0.95 {
                    // Glass
                    Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric::new(1.5)),
                    })
                } else {
                    // Diamond
                    Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric::new(2.4)),
                    })
                }
            };
            sphere_list.push(sphere);
        }
    }

    sphere_list.push(Arc::new(Sphere {
        center: Vec3::new(2.0, 1.0, -2.0),
        radius: 1.0,
        material: Arc::new(Dielectric::new(1.5)),
    }));
    sphere_list.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 1.0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
        }),
    }));
    sphere_list.push(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new(
            Arc::new(ConstantTexture::new(Vec3::new(0.5, 0.5, 0.5))),
            0.0,
        )),
    }));

    let ref mut list = HitableList { list: sphere_list };
    BvhNode::new(list, 0.0, 1.0)
}

fn create_cornell_box() -> BvhNode {
    #![allow(dead_code)]
    let red = Lambertian {
        albedo: Arc::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05))),
    };
    let white = Lambertian {
        albedo: Arc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))),
    };
    let green = Lambertian {
        albedo: Arc::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15))),
    };
    let light = DiffuseLight::new(Arc::new(ConstantTexture::new(Vec3::new(2.0, 2.0, 2.0))));
    let left_wall = Arc::new(YZRect {
        material: Arc::new(green),
        y_0: 0.0,
        y_1: 20.0,
        z_0: -10.0,
        z_1: 10.0,
        k: -10.0,
    });
    let right_wall = Arc::new(YZRect {
        material: Arc::new(red.clone()),
        y_0: 0.0,
        y_1: 20.0,
        z_0: -10.0,
        z_1: 10.0,
        k: 10.0,
    });
    let back_wall = Arc::new(XYRect {
        material: Arc::new(white.clone()),
        x_0: -10.0,
        x_1: 10.0,
        y_0: 0.0,
        y_1: 20.0,
        k: 10.0,
    });
    let front_wall = Arc::new(XYRect {
        material: Arc::new(white.clone()),
        x_0: -10.0,
        x_1: 10.0,
        y_0: 0.0,
        y_1: 20.0,
        k: -10.0,
    });
    let _front_light = Arc::new(XYRect {
        material: Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
            Vec3::new(2.0, 2.0, 2.0),
        )))),
        x_0: 0.0,
        x_1: 555.0,
        y_0: 0.0,
        y_1: 555.0,
        k: -605.0,
    });
    let floor = Arc::new(XZRect {
        material: Arc::new(white.clone()),
        x_0: -10.0,
        x_1: 10.0,
        z_0: -10.0,
        z_1: 10.0,
        k: 0.0,
    });
    let ceiling = Arc::new(XZRect {
        material: Arc::new(white.clone()),
        x_0: -10.0,
        x_1: 10.0,
        z_0: -10.0,
        z_1: 10.0,
        k: 20.0,
    });
    let _ceiling_light = Arc::new(XZRect {
        material: Arc::new(light.clone()),
        x_0: -7.5,
        x_1: 7.5,
        z_0: -7.5,
        z_1: 7.5,
        k: 20.0,
    });

    let _pedestal = Arc::new(AxisAlignedBlock::new(
        Vec3::new(-2.0, 0.0, -3.0),
        Vec3::new(2.0, 7.95, 1.0),
        Arc::new(Lambertian {
            albedo: Arc::new(ConstantTexture::new(Vec3::new(0.396, 0.263, 0.129))),
        }),
    ));

    let _teapot = load_obj_file(
        &Path::new("object-files/teapot.obj"),
        Arc::new(Dielectric::new(1.0)),
    );
    let lamp = load_obj_file(
        &Path::new("object-files/luxo/luxo_obj.obj"),
        Arc::new(Dielectric::new(1.0)),
    );
    let list: Vec<Arc<Hitable>> = vec![
        left_wall,
        Arc::new(FlipNormals::new(right_wall)),
        Arc::new(FlipNormals::new(back_wall)),
        front_wall,
        floor,
        Arc::new(FlipNormals::new(ceiling)),
        // _ceiling_light,
        Arc::new(RotateY::new(Arc::new(lamp), -70.0)),
        // Arc::new(Translate::new(Arc::new(_teapot), Vec3::new(0.0, 8.0, -1.5))),
        // _pedestal,
    ];

    BvhNode::new(&mut HitableList { list }, 0.0, 0.0)
}

fn create_debug_scene() -> BvhNode {
    #![allow(dead_code)]
    // Colors
    let white = Arc::new(Lambertian {
        albedo: Arc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))),
    }) as Arc<Material>;

    // Walls
    let back_wall = Arc::new(XYRect {
        material: Arc::clone(&white),
        x_0: -20.0,
        x_1: 20.0,
        y_0: 0.0,
        y_1: 40.0,
        k: -20.0,
    }) as Arc<Hitable>;
    let front_wall = Arc::new(Translate::new(
        Arc::new(FlipNormals::new(Arc::clone(&back_wall))),
        Vec3::new(00.0, 0.0, 40.0),
    ));
    let left_wall = Arc::new(YZRect {
        material: Arc::clone(&white),
        y_0: 0.0,
        y_1: 40.0,
        z_0: -20.0,
        z_1: 20.0,
        k: -20.0,
    }) as Arc<Hitable>;
    let right_wall = Arc::new(Translate::new(
        Arc::new(FlipNormals::new(Arc::clone(&left_wall))),
        Vec3::new(40.0, 0.0, 0.0),
    ));
    let ceiling = Arc::new(FlipNormals::new(Arc::new(XZRect {
        material: Arc::clone(&white),
        x_0: -20.0,
        x_1: 20.0,
        z_0: -20.0,
        z_1: 20.0,
        k: 40.0,
    })));
    let mut varnish = Glossy::new(Arc::new(ImageTexture::new("textures/wood.jpg")), 1.0);
    varnish.refractive_index = 1.66;
    let table_top = Arc::new(XZRect {
        material: Arc::new(varnish),
        x_0: -20.0,
        x_1: 20.0,
        z_0: -20.0,
        z_1: 20.0,
        k: 0.0,
    });

    // Objects
    let lamp = Arc::new(load_obj_file(
        &Path::new("object-files/luxo/luxo_obj.obj"),
        Arc::new(Dielectric::new(1.0)),
    ));
    let glass_ball = Arc::new(Sphere {
        center: Vec3::new(-2.5, 6.25, 0.75),
        radius: 1.5,
        material: Arc::new(Dielectric::new(1.525)),
    });
    let toy_ball = Arc::new(Sphere {
        center: Vec3::new(-2.0, 2.0, 0.75),
        radius: 2.0,
        material: Arc::new(Glossy::new(
            Arc::new(ImageTexture::new("textures/pixar_ball_copy.jpg")),
            0.25,
        )),
    });

    // Volumes
    let mist = Arc::new(ConstantMedium::new(
        Arc::new(Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 200.0,
            material: Arc::new(Dielectric::new(1.0)),
        }),
        0.0025,
        Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    ));

    let list: Vec<Arc<Hitable>> = vec![
        Arc::new(RotateY::new(lamp, -110.0)),
        table_top,
        glass_ball,
        left_wall,
        right_wall,
        back_wall,
        front_wall,
        ceiling,
        // mist,
        toy_ball,
    ];
    BvhNode::new(&mut HitableList { list }, 0.0, 1.0)
}

fn create_final_scene() -> BvhNode {
    #![allow(dead_code)]
    let mut rng = rand::thread_rng();

    // Ground definition
    let num_boxes = 20;
    let mut box_list: Vec<Arc<Hitable>> = vec![];
    let ground = Lambertian {
        albedo: Arc::new(ConstantTexture::new(Vec3::new(0.48, 0.83, 0.53))),
    };
    for i in 0..num_boxes {
        for j in 0..num_boxes {
            let width = 100.0;
            let (x_0, y_0, z_0) = (-1000.0 + i as f64 * width, 0.0, -1000.0 + j as f64 * width);
            let (x_1, y_1, z_1) = (x_0 + width, 100.0 * (0.01 + rng.gen::<f64>()), z_0 + width);
            box_list.push(Arc::new(AxisAlignedBlock::new(
                Vec3::new(x_0, y_0, z_0),
                Vec3::new(x_1, y_1, z_1),
                Arc::new(ground.clone()),
            )));
        }
    }

    // Light definition
    let light = DiffuseLight::new(Arc::new(ConstantTexture::new(Vec3::new(7.0, 7.0, 7.0))));
    let ceiling_light = Arc::new(XZRect {
        material: Arc::new(light),
        x_0: 123.0,
        x_1: 423.0,
        z_0: 147.0,
        z_1: 412.0,
        k: 554.0,
    });

    // Sphere definitions
    let fly_ball = Arc::new(MovingSphere {
        material: Arc::new(Lambertian {
            albedo: Arc::new(ConstantTexture::new(Vec3::new(0.7, 0.3, 0.1))),
        }),
        start_center: Vec3::new(400.0, 400.0, 200.0),
        end_center: Vec3::new(430.0, 400.0, 200.0),
        start_time: 0.0,
        end_time: 1.0,
        radius: 50.0,
    });
    let glass_ball = Arc::new(Sphere {
        material: Arc::new(Dielectric::new(1.5)),
        center: Vec3::new(260.0, 150.0, 45.0),
        radius: 50.0,
    });
    let metal_ball = Arc::new(Sphere {
        material: Arc::new(Metal::new(
            Arc::new(ConstantTexture::new(Vec3::new(0.8, 0.8, 0.9))),
            10.0,
        )),
        center: Vec3::new(0.0, 150.0, 145.0),
        radius: 50.0,
    });
    let marble_ball = Arc::new(Sphere {
        material: Arc::new(Lambertian {
            albedo: Arc::new(NoiseTexture::new(0.05, 8)),
        }),
        center: Vec3::new(220.0, 280.0, 300.0),
        radius: 80.0,
    });

    // Volume definitions
    let subsurface_boundary = Arc::new(Sphere {
        material: Arc::new(Dielectric::new(1.5)),
        center: Vec3::new(360.0, 150.0, 145.0),
        radius: 70.0,
    }) as Arc<Hitable>;
    let subsurface_volume = Arc::new(ConstantMedium::new(
        Arc::clone(&subsurface_boundary),
        0.5,
        Arc::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9))),
    ));
    let mist_boundary = Arc::new(Sphere {
        material: Arc::new(Dielectric::new(1.5)), // arbitrary material
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 5000.0,
    });
    let mist = Arc::new(VariableMedium::new(
        mist_boundary,
        0.0002,
        Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    ));

    // Sphere-cube definition
    let white = Lambertian {
        albedo: Arc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))),
    };
    let sphere_cube = (0..1000)
        .map(|_| {
            Arc::new(Sphere {
                material: Arc::new(white.clone()),
                center: Vec3::new(
                    165.0 * rng.gen::<f64>(),
                    165.0 * rng.gen::<f64>(),
                    165.0 * rng.gen::<f64>(),
                ),
                radius: 10.0,
            }) as Arc<Hitable>
        })
        .collect::<Vec<Arc<Hitable>>>();
    let sphere_cube = Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new(
                &mut HitableList { list: sphere_cube },
                0.0,
                1.0,
            )),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    let list: Vec<Arc<Hitable>> = vec![
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
        Arc::new(BvhNode::new(&mut HitableList { list: box_list }, 0.0, 1.0)),
    ];
    BvhNode::new(&mut HitableList { list }, 0.0, 1.0)
}

/// Recreates the "wada2" scene from smallpt (http://www.kevinbeason.com/smallpt/)
///
/// Note: The MAX_DEPTH variable needs to be increased (~50?) for this to properly render
fn wada() -> BvhNode {
    #![allow(dead_code)]
    let radius = 120.0;
    let theta = 30.0 * std::f64::consts::PI / 180.0;
    let distance = radius / theta.cos();
    let color = Vec3::new(0.275, 0.612, 0.949);

    let list: Vec<Arc<Hitable>> = vec![
        Arc::new(FlipNormals::new(Arc::new(Sphere {
            radius: 2.0 * 2.0 * radius * 2.0 * (2_f64 / 3_f64).sqrt()
                - radius * 2.0 * (2_f64 / 3_f64).sqrt() / 3.0,
            center: Vec3::new(50.0, 28.0, 62.0)
                + Vec3::new(0.0, 0.0, -radius * 2.0 * (2_f64 / 3_f64).sqrt() / 3.0),
            material: Arc::new(Metal::new(
                Arc::new(ConstantTexture::new(Vec3::new(0.5, 0.5, 0.5))),
                0.0,
            )),
        }))),
        Arc::new(Sphere {
            radius,
            center: Vec3::new(50.0, 28.0, 62.0)
                + Vec3::new(0.0, 0.0, -1.0) * radius * 2.0 * (2_f64 / 3_f64).sqrt(),
            material: Arc::new(Metal::new(
                Arc::new(ConstantTexture::new(Vec3::new(0.996, 0.996, 0.996))),
                0.0,
            )),
        }),
        Arc::new(Sphere {
            radius,
            center: Vec3::new(50.0, 28.0, 62.0) + Vec3::new(0.0, -1.0, 0.0) * distance,
            material: Arc::new(Metal::new_emitting(
                Arc::new(ConstantTexture::new(Vec3::new(0.996, 0.996, 0.996))),
                Arc::new(ConstantTexture::new(color * 6e-2)),
                0.0,
            )),
        }),
        Arc::new(Sphere {
            radius,
            center: Vec3::new(50.0, 28.0, 62.0)
                + Vec3::new(-(theta.cos()), theta.sin(), 0.0) * distance,
            material: Arc::new(Metal::new_emitting(
                Arc::new(ConstantTexture::new(Vec3::new(0.996, 0.996, 0.996))),
                Arc::new(ConstantTexture::new(color * 6e-2)),
                0.0,
            )),
        }),
        Arc::new(Sphere {
            radius,
            center: Vec3::new(50.0, 28.0, 62.0)
                + Vec3::new(theta.cos(), theta.sin(), 0.0) * distance,
            material: Arc::new(Metal::new_emitting(
                Arc::new(ConstantTexture::new(Vec3::new(0.996, 0.996, 0.996))),
                Arc::new(ConstantTexture::new(color * 6e-2)),
                0.0,
            )),
        }),
    ];
    BvhNode::new(&mut HitableList { list }, 0.0, 0.0)
}

/// Loads all the meshes defined in an OBJ file, and returns them in a
/// constructed BVH.
fn load_obj_file(file_path: &Path, mut material: Arc<Material>) -> BvhNode {
    match tobj::load_obj(file_path) {
        Ok((models, materials)) => {
            let mut meshes: Vec<Arc<Hitable>> = vec![];
            let mut img_textures = HashMap::new();
            for model in models {
                let mesh = model.mesh;
                if mesh.material_id.is_some() {
                    let mtl = &materials[mesh.material_id.unwrap()];
                    // FIXME: This is a hack to prevent trying to map to a transparent image
                    if !mtl.diffuse_texture.is_empty() && mtl.dissolve_texture.is_empty() {
                        if !img_textures.contains_key(&mtl.diffuse_texture) {
                            img_textures.insert(
                                mtl.diffuse_texture.to_string(),
                                Arc::new(ImageTexture::new(mtl.diffuse_texture.as_str()))
                                    as Arc<Texture>,
                            );
                        }
                        material = Arc::new(Lambertian {
                            albedo: Arc::clone(img_textures.get(&mtl.diffuse_texture).unwrap()),
                        });
                    } else {
                        // Refractive Index
                        let ior = if mtl.optical_density != 1.0 {
                            mtl.optical_density as f64
                        } else if mtl.shininess != 0.0 {
                            1.45
                        } else {
                            1.0
                        };
                        // Emittance
                        let emittance_color = if mtl.unknown_param.contains_key("Ke") {
                            Vec3::from_vec(
                                mtl.unknown_param
                                    .get("Ke")
                                    .unwrap()
                                    .split_whitespace()
                                    .map(|s| f64::from_str(s).unwrap())
                                    .collect(),
                            )
                        } else {
                            Vec3::new(0.0, 0.0, 0.0)
                        };
                        material = Arc::new(Glossy {
                            albedo: Arc::new(ConstantTexture::new(Vec3::new(
                                mtl.diffuse[0] as f64,
                                mtl.diffuse[1] as f64,
                                mtl.diffuse[2] as f64,
                            ))),
                            specular_albedo: Arc::new(ConstantTexture::new(Vec3::new(
                                mtl.specular[0] as f64,
                                mtl.specular[1] as f64,
                                mtl.specular[2] as f64,
                            ))),
                            emittance_albedo: Arc::new(ConstantTexture::new(emittance_color)),
                            glossiness: (mtl.shininess / 1_000.0) as f64,
                            refractive_index: ior,
                        });
                    }
                }
                // all vertices in the mesh
                let vertices: Vec<Vec3> = mesh
                    .positions
                    .chunks(3)
                    .map(|i| Vec3::new(i[0] as f64, i[1] as f64, i[2] as f64))
                    .collect();
                // all vertex-normals in the mesh
                let mut normals: Vec<Vec3> = vec![];
                if !mesh.normals.is_empty() {
                    normals = mesh
                        .normals
                        .chunks(3)
                        .map(|i| Vec3::new(i[0] as f64, i[1] as f64, i[2] as f64))
                        .collect();
                }
                // all texture coordinates in the mesh
                let mut texcoords: Vec<(f64, f64)> = vec![];
                if !mesh.texcoords.is_empty() {
                    texcoords = mesh
                        .texcoords
                        .chunks(2)
                        .map(|i| (i[0] as f64, i[1] as f64))
                        .collect();
                }
                // Construct faces
                let faces: Vec<Arc<Hitable>> = mesh
                    .indices
                    .chunks(3)
                    .map(|i| {
                        let mut face = Polygon::new(
                            vec![
                                vertices[i[0] as usize],
                                vertices[i[1] as usize],
                                vertices[i[2] as usize],
                            ],
                            Arc::clone(&material),
                        );
                        if !normals.is_empty() {
                            face.vertex_normals = Some(vec![
                                normals[i[0] as usize],
                                normals[i[1] as usize],
                                normals[i[2] as usize],
                            ]);
                        }
                        if !texcoords.is_empty() {
                            face.texture_coords = Some(vec![
                                texcoords[i[0] as usize],
                                texcoords[i[1] as usize],
                                texcoords[i[2] as usize],
                            ]);
                        }
                        Arc::new(face) as Arc<Hitable>
                    })
                    .collect();
                meshes.push(Arc::new(PolygonMesh::new(faces)));
            }
            BvhNode::new(&mut HitableList { list: meshes }, 0.0, 0.0)
        }
        Err(e) => {
            println!("Failed to load file {:?}: {:?}", file_path, e);
            BvhNode::new(&mut HitableList { list: vec![] }, 0.0, 0.0)
        }
    }
}

fn main() {
    let num_threads: usize = num_cpus::get() - 1;
    let num_x = 264 * 2;
    let num_y = 180 * 2;
    // let num_x = 300;
    // let num_y = 300;
    // n and m are the dimensions of the subpixel grid generated for anti-aliasing
    // let (n, m) = (2, 2);
    let (n, m) = (40, 40);
    let range = Uniform::new(0.0, 1.0);
    let mut img_buff = image::ImageBuffer::new(num_x, num_y);
    let look_from = Vec3::new(-3.0, 2.0, 20.0);
    let look_in = Vec3::new(0.0, 0.125, -1.0);
    let camera = Camera::new(
        look_from,                   // Camera origin
        look_in,                     // Camera view direction
        Vec3::new(0.0, 1.0, 0.0),    // Camera "up" direction
        40.0,                        // Vertical FOV
        num_x as f64 / num_y as f64, // Aspect ratio
        0.0,                         // Aperture
        10.0,                        // Focus Distance
        0.0,                         // Shutter open time
        1.0,                         // Shutter close time
    );

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner().template("{spinner} {msg}: [{elapsed_precise}] "),
    );
    spinner.set_message("Performing Scene Construction");
    spinner.enable_steady_tick(100);
    let world = Arc::new(create_debug_scene());
    // let world = Arc::new(create_cornell_box());
    spinner.finish_with_message("Scene Construction Completed");

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
                    // Correlated Multi-Jittered Sampling
                    // Source: (http://graphics.pixar.com/library/MultiJitteredSampling/paper.pdf)
                    // Step 1: Produce the canonical arrangement
                    let mut sample_pattern: Vec<(f64, f64)> = vec![(0.0, 0.0); n * m];
                    for j in 0..n {
                        for i in 0..m {
                            sample_pattern[j * m + i].0 = (i as f64
                                + (j as f64 * range.sample(&mut rng)) / n as f64)
                                / m as f64;
                            sample_pattern[j * m + i].1 = (j as f64
                                + (i as f64 * range.sample(&mut rng)) / m as f64)
                                / n as f64;
                        }
                    }
                    // Step 2: Shuffle the arrangement
                    for j in 0..n {
                        for k in 0..m {
                            let i = (j as f64 + range.sample(&mut rng) * (n - j) as f64) as usize;
                            let a = sample_pattern[j * m + i].0;
                            let b = sample_pattern[k * m + i].0;
                            sample_pattern[j * m + i].0 = b;
                            sample_pattern[k * m + i].0 = a;
                        }
                    }
                    for i in 0..m {
                        for k in 0..n {
                            let j = (i as f64 + range.sample(&mut rng) * (m - i) as f64) as usize;
                            let a = sample_pattern[j * m + i].1;
                            let b = sample_pattern[j * m + k].1;
                            sample_pattern[j * m + i].1 = b;
                            sample_pattern[j * m + k].1 = a;
                        }
                    }
                    // Step 3: Use the sample arrangement
                    for sample in sample_pattern {
                        let ray = camera.create_ray(
                            (x as f64 + sample.0) / (num_x as f64),
                            (y as f64 + sample.1) / (num_y as f64),
                        );
                        _color += get_color(&ray, &_world, 0);
                    }
                    _color
                }));
            }
            for thread in child_threads {
                color += thread.join().unwrap();
            }
            color /= (n * m * num_threads) as f64;
            let r = (color.r().min(1.0).sqrt() * 255.99) as u8;
            let g = (color.g().min(1.0).sqrt() * 255.99) as u8;
            let b = (color.b().min(1.0).sqrt() * 255.99) as u8;
            let pixel = image::Rgb([r, g, b]);
            // Invert y coordinate
            img_buff.put_pixel(x, (num_y - 1) - y, pixel);
        }
        progress_bar.inc(num_x as u64);
    }
    progress_bar.println("Scene Tracing Completed.");
    progress_bar.finish();

    let path = &Path::new("output.png");
    match File::create(path) {
        Ok(_) => {
            let _ = image::ImageRgb8(img_buff).save(path);
        }
        Err(e) => println!("Failed to open file: {:?}", e),
    }
}
