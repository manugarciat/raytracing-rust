use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};


mod vec3;
mod color;
mod ray;


fn ray_color(r: &Ray) -> Color {
    Color::new_values(1.0, 0.0, 0.0)
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = (image_width as f64 / aspect_ratio) as usize;
    if (image_height < 1) { image_height = 1 }

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point3 = Point3::new_values(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vec3 = Vec3::new_values(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new_values(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vec3::new_values(0.0, 0.0, focal_length)
        - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    // Encabezado archivo .ppm
    println!("{}", format!("P3\n{} {}{}", image_width, image_height, "\n255"));

    for j in 0..image_height {
        eprintln!("{}", format!("\rScanlines remaining: {}", image_height - j));
        for i in 0..image_width {
            let pixel_center: Vec3 =
                pixel00_loc + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;

            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new_values(camera_center, ray_direction);

            let pixel_color: Color = ray_color(&r);
            write_color(pixel_color);
        }
    }
    eprintln!("\rDone.");
}
