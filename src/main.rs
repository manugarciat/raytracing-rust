use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::vec3::{dot, unit_vector, Point3, Vec3};

mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    // Vector desde el centro de la esfera al origen del rayo
    let oc: Vec3 = *center - *ray.origin();
    // Coeficiente cuadrático de la ecuación cuadrática
    let a: f64 = ray.direction().length_squared();
    // Mitad del coeficiente lineal
    let h: f64 = dot(*ray.direction(), oc);
    // Término constante
    let c: f64 = oc.length_squared() - radius * radius;
    // Calculamos el discriminante para saber si hay intersección
    let discriminant: f64 = h * h - a * c;

    return if discriminant < 0.0 {
        -1. // No hay intersección
    } else {
        (h - discriminant.sqrt()) / a
    }; // nos quedamos con el hit mas cercano
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0., 0., -1.), 0.5, r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vec3::new(0., 0., -1.));
        return 0.5 * Color::rgb(n.x() + 1., n.y() + 1., n.z() + 1.);
    }
    let unit_direction = unit_vector(*r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::rgb(1.0, 1.0, 1.0) + a * Color::rgb(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = (image_width as f64 / aspect_ratio) as usize;
    if (image_height < 1) {
        image_height = 1
    }

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    // Encabezado archivo .ppm
    println!(
        "{}",
        format!("P3\n{} {}{}", image_width, image_height, "\n255")
    );

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
