use lib::utilities::color::Color;
use std::{fs::File, io::Write};

fn main() {
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html

    let image_height = 256;
    let image_width = 256;
    let file_path = "image_test.ppm";

    let mut file = File::create(&file_path).unwrap();

    writeln!(file, "P3\n{} {}\n255", image_width, image_height).unwrap();
    for y_index in 0..image_height {
        println!("Remaining scanlines: {}", image_height - y_index); // Adding a Progress Indicator
        for x_index in 0..image_width {
            let r = x_index as f64 / (image_width - 1) as f64;
            let g = y_index as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let pixel_color = Color::new(r, g, b);
            let write_res = pixel_color.write_color(&mut file);
            match write_res {
                Err(e) => println!("Error in writing result to file: {}", e),
                _ => (), // The () is just the unit value, so nothing will happen
            }
        }
    }

    println!("Done!");
}
