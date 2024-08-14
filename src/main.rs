use lib::utilities::{
    camera::Camera,
    color::Color,
    geometry::{Hittable, Sphere},
    material::{Dielectric, Lambertian, Metal},
    point::Point3,
    vector3::Vector3,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 384;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;
const VERTICAL_FOV: f64 = 60.0;

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html

    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Dielectric::new(1.33));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.7));
    let material_bubble = Box::new(Dielectric::new(1.0 / 1.33));

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));

    // Camera
    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = ASPECT_RATIO;
    cam.image_width = IMAGE_WIDTH;
    cam.samples_per_pixel = SAMPLES_PER_PIXEL;
    cam.max_depth = MAX_DEPTH;

    cam.vertical_field_of_view = VERTICAL_FOV; // Zooms in/out of the image
    cam.look_from = Point3::new(-2.0, 2.0, 1.0);
    cam.look_at = Point3::new(0.0, 0.0, -1.0);
    cam.vertical_camera_up = Vector3::new(0.0, 1.0, 0.0);

    cam.render(world);
}
