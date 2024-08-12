use lib::utilities::{
    camera::Camera,
    point::Point3,
    sphere::{HittableObjects, Sphere},
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 384;
const SAMPLES_PER_PIXEL: i32 = 100;

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html

    // World
    let mut world: HittableObjects = HittableObjects::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = ASPECT_RATIO;
    cam.image_width = IMAGE_WIDTH;
    cam.samples_per_pixel = SAMPLES_PER_PIXEL;
    cam.render(&world);
}
