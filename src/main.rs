use rand::Rng;

use lib::utilities::{
    camera::Camera,
    color::Color,
    geometry::{Hittable, Sphere},
    material::{Dielectric, Lambertian, Metal},
    point::Point3,
    vector3::Vector3,
};

const NUMBER_BALLS: i32 = 5;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 160;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;
const VERTICAL_FOV: f64 = 40.0;

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html

    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    // Scene - ground
    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.8)));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    // Scene - small balls (random)
    for x_index in -NUMBER_BALLS..NUMBER_BALLS {
        for y_index in -NUMBER_BALLS..NUMBER_BALLS {
            let choose_material_random: f64 = rand::thread_rng().r#gen::<f64>();
            let center = Point3::new(
                (x_index as f64) + (0.9 * rand::thread_rng().r#gen::<f64>()),
                0.2,
                (y_index as f64) + (0.9 * rand::thread_rng().r#gen::<f64>()),
            );
            if choose_material_random < 0.6 {
                // Lambertian
                let material_lambertian = Box::new(Lambertian::new(Color::new(
                    rand::thread_rng().r#gen::<f64>(),
                    rand::thread_rng().r#gen::<f64>(),
                    rand::thread_rng().r#gen::<f64>(),
                )));
                world.push(Box::new(Sphere::new(center, 0.2, material_lambertian)));
            } else if choose_material_random < 0.85 {
                // Metal
                let material_metal = Box::new(Metal::new(
                    Color::new(
                        rand::thread_rng().r#gen::<f64>(),
                        rand::thread_rng().r#gen::<f64>(),
                        rand::thread_rng().r#gen::<f64>(),
                    ),
                    rand::thread_rng().r#gen::<f64>(),
                ));
                world.push(Box::new(Sphere::new(center, 0.2, material_metal)));
            } else {
                // Glass
                let material_glass = Box::new(Dielectric::new(1.33));
                world.push(Box::new(Sphere::new(center, 0.2, material_glass)));
            }
        }
    }

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
