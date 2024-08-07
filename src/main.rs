use std::{fs::File, io::Write};

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html

    let image_height = 255;
    let image_width = 255;
    let file_path = "image_test.ppm";

    let mut file = File::create(&file_path).unwrap();

    for x_index in 1..image_height {
        for y_index in 1..image_width {
            let r = x_index as f64 / (image_width - 1) as f64;
            let g = y_index as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writeln!(file, "{} {} {}", ir, ig, ib).unwrap();
        }
    }
}
