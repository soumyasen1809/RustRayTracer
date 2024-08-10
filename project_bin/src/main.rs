use lib::utilities::{
    color::Color,
    point::Point3,
    ray::Ray,
    sphere::{HittableObjects, Sphere},
    vector3::Vector3,
};
use std::{fs::File, io::Write};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const FOCAL_LENGTH: f64 = 1.0;

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html

    // Image
    let image_width: i32 = 384;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height: i32 = (image_width as f64 / ASPECT_RATIO) as i32;
    if image_height < 1 {
        image_height = 1
    }

    // World
    let mut world: HittableObjects = HittableObjects::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u: Vector3 = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vector3 = Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u: Vector3 = viewport_u / (image_width as f64);
    let pixel_delta_v: Vector3 = viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel
    let viewport_origin: Point3 = camera_center
        - Vector3::new(0.0, 0.0, FOCAL_LENGTH)
        - (viewport_u / 2.0)
        - (viewport_v / 2.0);

    let pixel00_loc: Point3 = viewport_origin + ((pixel_delta_u + pixel_delta_v) * 0.5);

    // Render and write to file
    let file_path = "image_test.ppm";
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "P3\n{} {}\n255", image_width, image_height).unwrap();
    for y_index in 0..image_height {
        println!("Remaining scanlines: {}", image_height - y_index); // Adding a Progress Indicator
        for x_index in 0..image_width {
            let pixel_center: Point3 = pixel00_loc
                + (pixel_delta_u * (x_index as f64))
                + (pixel_delta_v * (y_index as f64));

            let ray_direction: Vector3 = (pixel_center - camera_center).as_vec();
            let ray_sent: Ray = Ray::new(camera_center, ray_direction);
            let pixel_color: Color = ray_sent.ray_color(&world);

            let write_res = pixel_color.write_color(&mut file);
            match write_res {
                Err(e) => println!("Error in writing result to file: {}", e),
                _ => (), // The () is just the unit value, so nothing will happen
            }
        }
    }

    println!("Done!");
}
