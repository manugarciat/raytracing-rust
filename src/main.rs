use crate::vec3::{cross, dot, unit_vector, Vec3};

mod vec3;

// use vec3::*;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render
    println!("{}", format!("P3\n{} {}{}", image_width, image_height, "\n255"));

    // for j in 0..image_height {
    //     eprintln!("{}", format!("\rScanlines remaining: {}", image_height - j));
    //     for i in 0..image_width {
    //         let r = i as f64 / (image_width - 1) as f64;
    //         let g = j as f64 / (image_height - 1) as f64;
    //         let b = 0.0;
    //
    //         let ir = (255.99 * r) as i32;
    //         let ig = (255.99 * g) as i32;
    //         let ib = (255.99 * b) as i32;
    //         println!("{} {} {}", ir, ig, ib);
    //     }
    // }
    // eprintln!("\rDone.");


    //pruebas Vec3
    let v1: Vec3 = Vec3::new_values(1.2, 3.4, 8.0);
    let v2: Vec3 = Vec3::new_values(3.4, 5.6, 2.0);
    eprintln!("v1: {:?}", v1);
    eprintln!("v2: {:?}", v2);

    let mut suma = Vec3::new_values(1.0, 1.0, 1.0);
    suma *= 2.0;
    suma[1] = 3.2;
    let x = suma[1];
    println!("{:?}", suma.z());
    let f = cross(v1, v2);
    println!("{:?}", f);
    println!("{:?}", unit_vector(suma));

}
