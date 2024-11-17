use crate::color::{write_color, Color};
use crate::vec3::{cross, dot, unit_vector, Vec3, Point3};

mod vec3;
mod color;
mod ray;
// use vec3::*;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render
    println!("{}", format!("P3\n{} {}{}", image_width, image_height, "\n255"));

    for j in 0..image_height {
        eprintln!("{}", format!("\rScanlines remaining: {}", image_height - j));
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;
            let pixel_color: Color = Color::new_values(r, g, b);

            write_color(pixel_color);
        }
    }
    eprintln!("\rDone.");
}
