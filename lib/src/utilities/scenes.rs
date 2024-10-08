use rand::Rng;
use rayon::prelude::*;
use std::{collections::HashMap, fs};

use super::{
    color::Color,
    geometry::{Hittable, Sphere},
    material::{Dielectric, Lambertian, Metal},
    point::Point3,
};

const NUMBER_BALLS: i32 = 7;
const SCENE_FILE_PATH: &str = "scene_data.json";

fn translate_color_to_scale(color_component: f64) -> f64 {
    color_component.clamp(0.0, 256.0) / 256.0
}

fn read_from_json(world: &mut Vec<Box<dyn Hittable>>) {
    let scenes_file_path = SCENE_FILE_PATH.to_owned();
    let file = fs::File::open(scenes_file_path).expect("Could not open file");
    let json_data: HashMap<String, Vec<HashMap<String, serde_json::Value>>> =
        serde_json::from_reader(file).expect("File is not proper JSON");
    let json_parse_ball = json_data.get("Ball").expect("Can't read Ball data");

    for ball in json_parse_ball.iter() {
        {
            if let Some(material_str) = ball["material"]["type"].as_str() {
                if material_str == "lambertian" {
                    let color_obj = Color::new(
                        translate_color_to_scale(ball["color"]["r"].as_f64().unwrap_or_default()),
                        translate_color_to_scale(ball["color"]["g"].as_f64().unwrap_or_default()),
                        translate_color_to_scale(ball["color"]["b"].as_f64().unwrap_or_default()),
                    );
                    let material_obj = Box::new(Lambertian::new(color_obj));

                    let center_obj = Point3::new(
                        ball["center"]["x"].as_f64().unwrap_or_default(),
                        ball["center"]["y"].as_f64().unwrap_or_default(),
                        ball["center"]["z"].as_f64().unwrap_or_default(),
                    );
                    let radius_obj = ball["radius"].as_f64().unwrap_or_default();

                    world.push(Box::new(Sphere::new(center_obj, radius_obj, material_obj)));
                } else if material_str == "metal" {
                    let color_obj = Color::new(
                        translate_color_to_scale(ball["color"]["r"].as_f64().unwrap_or_default()),
                        translate_color_to_scale(ball["color"]["g"].as_f64().unwrap_or_default()),
                        translate_color_to_scale(ball["color"]["b"].as_f64().unwrap_or_default()),
                    );
                    let fuzz_obj = ball["material"]["fuzz"].as_f64().unwrap_or_default();
                    let material_obj = Box::new(Metal::new(color_obj, fuzz_obj));

                    let center_obj = Point3::new(
                        ball["center"]["x"].as_f64().unwrap_or_default(),
                        ball["center"]["y"].as_f64().unwrap_or_default(),
                        ball["center"]["z"].as_f64().unwrap_or_default(),
                    );
                    let radius_obj = ball["radius"].as_f64().unwrap_or_default();

                    world.push(Box::new(Sphere::new(center_obj, radius_obj, material_obj)));
                } else if material_str == "dielectric" {
                    let rf_index_obj = ball["material"]["ref_idx"].as_f64().unwrap_or_default();
                    let material_obj = Box::new(Dielectric::new(rf_index_obj));

                    let center_obj = Point3::new(
                        ball["center"]["x"].as_f64().unwrap_or_default(),
                        ball["center"]["y"].as_f64().unwrap_or_default(),
                        ball["center"]["z"].as_f64().unwrap_or_default(),
                    );
                    let radius_obj = ball["radius"].as_f64().unwrap_or_default();

                    world.push(Box::new(Sphere::new(center_obj, radius_obj, material_obj)));
                } else {
                    println!("Wrong Material");
                }
            }
        }
    }
}

pub fn generate_scene(world: &mut Vec<Box<dyn Hittable>>) {
    // World

    // Scene - ground
    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.8)));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    // Scene - small balls (random)
    let spheres_scene: Vec<Box<dyn Hittable>> = (-NUMBER_BALLS..NUMBER_BALLS)
        .into_par_iter()
        .flat_map(|x_index| {
            (-NUMBER_BALLS..NUMBER_BALLS)
                .into_par_iter()
                .map(|y_index| {
                    let choose_material_random: f64 = rand::thread_rng().r#gen::<f64>();
                    let center = Point3::new(
                        (x_index as f64) + (0.9 * rand::thread_rng().r#gen::<f64>()),
                        0.2,
                        (y_index as f64) + (0.9 * rand::thread_rng().r#gen::<f64>()),
                    );
                    if choose_material_random < 0.4 {
                        // Lambertian
                        let material_lambertian = Box::new(Lambertian::new(Color::new(
                            rand::thread_rng().r#gen::<f64>(),
                            rand::thread_rng().r#gen::<f64>(),
                            rand::thread_rng().r#gen::<f64>(),
                        )));
                        Box::new(Sphere::new(center, 0.2, material_lambertian)) as Box<dyn Hittable>
                    } else if choose_material_random < 0.8 {
                        // Metal
                        let material_metal = Box::new(Metal::new(
                            Color::new(
                                rand::thread_rng().r#gen::<f64>(),
                                rand::thread_rng().r#gen::<f64>(),
                                rand::thread_rng().r#gen::<f64>(),
                            ),
                            rand::thread_rng().r#gen::<f64>(),
                        ));
                        Box::new(Sphere::new(center, 0.2, material_metal)) as Box<dyn Hittable>
                    } else {
                        // Glass
                        let material_glass = Box::new(Dielectric::new(1.33));
                        Box::new(Sphere::new(center, 0.2, material_glass)) as Box<dyn Hittable>
                    }
                })
                .collect::<Vec<Box<dyn Hittable>>>()
        })
        .collect();

    world.extend(spheres_scene);

    // Scene - big balls with Glass material
    let material_glass = Box::new(Dielectric::new(1.0 / 1.55));
    world.push(Box::new(Sphere::new(
        Point3::new(8.0, 1.0, 0.0),
        1.0,
        material_glass,
    )));
    let material_bubble = Box::new(Dielectric::new(1.55));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_bubble,
    )));

    // Scene - big ball with Lambertian material
    let material_lambertian = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_lambertian,
    )));

    // Scene - big ball with Metal material
    let material_metal = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.2));
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_metal,
    )));

    // Scene - Load from Json
    read_from_json(world);
}
