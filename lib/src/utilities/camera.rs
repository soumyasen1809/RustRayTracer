use rand::Rng;

use super::{
    color::Color, geometry::Hittable, interval::Interval, material::Scatter, point::Point3,
    ray::Ray, vector3::Vector3,
};
use std::{fs::File, io::Write};

#[derive(Default, Clone)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32, // Maximum number of ray bounces
    pub vertical_field_of_view: f64,
    image_height: i32,
    camera_center: Point3,
    pixel00_loc: Point3,      // Location of pixel 0, 0
    pixel_delta_u: Vector3,   // Offset to pixel to the right
    pixel_delta_v: Vector3,   // Offset to pixel below
    pixel_samples_scale: f64, //  Color scale factor for a sum of pixel samples
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            vertical_field_of_view: 90.0,
            ..Default::default() // this is possible using the derive(Default)
        }
    }

    pub fn render(&mut self, world: Vec<Box<dyn Hittable>>) {
        self.initialize();

        // Render and write to file
        let file_path = "image_test.ppm";
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();
        for y_index in 0..self.image_height {
            println!("Remaining scanlines: {}", self.image_height - y_index); // Adding a Progress Indicator
            for x_index in 0..self.image_width {
                let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray_sent: Ray = self.get_ray(x_index, y_index);
                    pixel_color += Self::ray_color(ray_sent, self.max_depth, &world[..]);
                }

                let write_res = (pixel_color * self.pixel_samples_scale).write_color(&mut file);
                match write_res {
                    Err(e) => println!("Error in writing result to file: {}", e),
                    _ => (), // The () is just the unit value, so nothing will happen
                }
            }
        }

        println!("Done!");
    }

    fn initialize(&mut self) {
        // Image
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1
        }

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.camera_center = Point3::new(0.0, 0.0, 0.0);

        // Camera
        let focal_length: f64 = 1.0;
        let theta: f64 = self.vertical_field_of_view.to_radians();
        let viewport_height: f64 = 2.0 * focal_length * (theta / 2.0).tan();
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u: Vector3 = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vector3 = Vector3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel
        let viewport_origin: Point3 = self.camera_center
            - Vector3::new(0.0, 0.0, focal_length)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);

        self.pixel00_loc = viewport_origin + ((self.pixel_delta_u + self.pixel_delta_v) * 0.5);
    }

    fn ray_color(ray: Ray, depth: i32, world: &[Box<dyn Hittable>]) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::default();
        }

        if let Some(hit) = world.hit(ray, Interval::new(0.001, std::f64::INFINITY)) {
            if let Some(scatter) = hit.material.scatter(ray, &hit) {
                let Scatter {
                    scattered_ray,
                    attenuation,
                } = scatter;
                return (Self::ray_color(scattered_ray, depth - 1, world)) * attenuation;
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        // Color the background blue - Linear blending
        let unit_direction: Vector3 = ray.get_direction().unit_vector();
        let a: f64 = 0.5 * (unit_direction.get_y() + 1.0);
        (Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Color::new(0.5, 0.7, 1.0) * a)
    }

    /// Construct a camera ray originating from the origin
    /// and directed at randomly sampled
    /// point around the pixel location i, j.
    fn get_ray(&self, loc_x: i32, loc_y: i32) -> Ray {
        let offset: Vector3 = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (loc_x as f64 + offset.get_x()))
            + (self.pixel_delta_v * (loc_y as f64 + offset.get_y()));

        let ray_direction = (pixel_sample - self.camera_center).as_vec();

        Ray::new(self.camera_center, ray_direction)
    }

    /// Returns the vector to a random point in the
    /// [-.5,-.5] to [+.5,+.5] unit square.
    fn sample_square() -> Vector3 {
        Vector3::new(
            rand::thread_rng().r#gen::<f64>() - 0.5,
            rand::thread_rng().r#gen::<f64>() - 0.5,
            0.0,
        )
    }
}
