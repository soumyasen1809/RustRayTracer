use std::fs;

use rand::Rng;

use lib::utilities::{
    camera::Camera,
    color::Color,
    geometry::{Hittable, Sphere},
    material::{Dielectric, Lambertian, Metal},
    point::Point3,
    vector3::Vector3,
};
use rayon::prelude::*;

const NUMBER_BALLS: i32 = 5;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 80;
const SAMPLES_PER_PIXEL: i32 = 20;
const MAX_DEPTH: i32 = 50;
const VERTICAL_FOV: f64 = 40.0;

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html

    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    // ------------------------------------------------------

    let scenes_file_path = "scene_data.json".to_owned();
    let file = fs::File::open(scenes_file_path).expect("Could not open file");
    let json_data: serde_json::Value =
        serde_json::from_reader(file).expect("File is not proper JSON");
    let json_parse_ball = json_data.get("Ball").expect("Cant read Ball data");
    if let Some(ball) = json_parse_ball.get(0) {
        if let Some(material_str) = ball["material"]["type"].as_str() {
            // ["type"].as_str() needed else output is: Object {"type": String("lambertian")}
            println!("Material of the first ball: {:?}", material_str);
            if material_str == "lambertian" {
                let color_obj = Color::new(
                    ball["color"]["r"].as_f64().unwrap(),
                    ball["color"]["g"].as_f64().unwrap(),
                    ball["color"]["b"].as_f64().unwrap(),
                );
                let material_obj = Box::new(Lambertian::new(color_obj));

                let center_obj = Point3::new(
                    ball["center"]["x"].as_f64().unwrap(),
                    ball["center"]["y"].as_f64().unwrap(),
                    ball["center"]["z"].as_f64().unwrap(),
                );
                let radius_obj = ball["radius"].as_f64().unwrap();
                println!(
                    "{:?}, {}, {}",
                    color_obj.get_r(),
                    center_obj.get_x(),
                    radius_obj
                );

                world.push(Box::new(Sphere::new(center_obj, radius_obj, material_obj)));
            }
            if material_str == "metal" {
                let color_obj = Color::new(
                    ball["color"]["r"].as_f64().unwrap(),
                    ball["color"]["g"].as_f64().unwrap(),
                    ball["color"]["b"].as_f64().unwrap(),
                );
                let fuzz_obj = ball["material"]["fuzz"].as_f64().unwrap();
                let material_obj = Box::new(Metal::new(color_obj, fuzz_obj));

                let center_obj = Point3::new(
                    ball["center"]["x"].as_f64().unwrap(),
                    ball["center"]["y"].as_f64().unwrap(),
                    ball["center"]["z"].as_f64().unwrap(),
                );
                let radius_obj = ball["radius"].as_f64().unwrap();
                println!(
                    "{:?}, {}, {}",
                    color_obj.get_r(),
                    center_obj.get_x(),
                    radius_obj
                );

                world.push(Box::new(Sphere::new(center_obj, radius_obj, material_obj)));
            }
            if material_str == "dielectric" {
                let rf_index_obj = ball["material"]["ref_idx"].as_f64().unwrap();
                let material_obj = Box::new(Dielectric::new(rf_index_obj));

                let center_obj = Point3::new(
                    ball["center"]["x"].as_f64().unwrap(),
                    ball["center"]["y"].as_f64().unwrap(),
                    ball["center"]["z"].as_f64().unwrap(),
                );
                let radius_obj = ball["radius"].as_f64().unwrap();
                println!("{:?}, {}", center_obj.get_x(), radius_obj);

                world.push(Box::new(Sphere::new(center_obj, radius_obj, material_obj)));
            }
        }
    }

    // ------------------------------------------------------

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
                        return Box::new(Sphere::new(center, 0.2, material_lambertian))
                            as Box<dyn Hittable>;
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
                        return Box::new(Sphere::new(center, 0.2, material_metal))
                            as Box<dyn Hittable>;
                    } else {
                        // Glass
                        let material_glass = Box::new(Dielectric::new(1.33));
                        return Box::new(Sphere::new(center, 0.2, material_glass))
                            as Box<dyn Hittable>;
                    }
                })
                .collect::<Vec<Box<dyn Hittable>>>()
        })
        .collect();

    world.extend(spheres_scene);
    // If you don't need to use spheres_scene after pushing its contents to world, you can move the entire vector.
    // Explanation: world.extend(spheres_scene) moves the contents of spheres_scene into world. The extend method consumes the vector, avoiding the need for cloning or referencing, and effectively transfers ownership of each Box<dyn Hittable> to world.
    // Use moving (via extend) if you want to transfer ownership and don't need to use spheres_scene afterward, which is more efficient because it avoids unnecessary duplication.

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

    // Camera
    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = ASPECT_RATIO;
    cam.image_width = IMAGE_WIDTH;
    cam.samples_per_pixel = SAMPLES_PER_PIXEL;
    cam.max_depth = MAX_DEPTH;

    cam.vertical_field_of_view = VERTICAL_FOV; // Zooms in/out of the image
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vertical_camera_up = Vector3::new(0.0, 1.0, 0.0);

    cam.render(world);
}
