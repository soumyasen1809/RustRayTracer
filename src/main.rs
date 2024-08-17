use lib::utilities::{camera::Camera, geometry::Hittable, point::Point3, scenes, vector3::Vector3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 1600;
const SAMPLES_PER_PIXEL: i32 = 200;
const MAX_DEPTH: i32 = 80;
const VERTICAL_FOV: f64 = 40.0;

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    scenes::generate_scene(&mut world);

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
