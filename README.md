# Ray Tracing in One Weekend (Rust Implementation)

This repository contains my Rust implementation of the ray tracing exercises described in the book *[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)* by **Peter Shirley**, **Trevor David Black**, and **Steve Hollasch**.

The project focuses on building a simple, yet powerful ray tracing renderer from scratch, exploring foundational computer graphics concepts such as rendering, light, color, and geometric intersections.

---

## 📖 **About the Book**

*Ray Tracing in One Weekend* is a hands-on introduction to ray tracing, a rendering technique used to generate photorealistic images by simulating the way light interacts with objects. The book walks through the creation of a small ray tracer in C++.

This project reimplements the exercises and concepts using **Rust**, emphasizing safety, concurrency, and performance.

---

## 🚀 **What This Project Does**

This implementation currently includes:

1. **Ray-Sphere Intersection**:
   - Rendering simple 3D scenes with spheres and a ground plane.
   - Calculating intersections between rays and objects.

2. **Materials and Shading**:
   - Adding diffuse, reflective, and transparent surfaces.
   - Simulating realistic lighting effects such as shadows and highlights.

3. **Camera**:
   - Implementing a virtual camera with adjustable parameters like field of view and depth of field.

4. **Multithreading**:
   - Leveraging Rust's concurrency features to render scenes faster.

5. **Output**:
   - Generating `.ppm` image files that showcase the rendered scenes.

---

## 📂 **Project Structure**

```plaintext
├── src/
│   ├── main.rs           # Entry point for the ray tracer
│   ├── camera.rs         # Camera implementation
│   ├── hittable.rs       # Traits and structures for objects
│   ├── material.rs       # Material definitions (diffuse, reflective, etc.)
│   ├── ray.rs            # Ray structure and utility methods
│   ├── sphere.rs         # Sphere object and ray intersection logic
│   ├── vec3.rs           # 3D vector operations
│   └── scene.rs          # Scene composition and rendering logic
├── images/               # Rendered output images
├── Cargo.toml            # Rust project dependencies
└── README.md             # This file
