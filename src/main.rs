use std::rc::Rc;

mod camera;
mod color;
mod material;
mod object;
mod ray;
mod util;
mod vec3;

pub use crate::camera::Camera;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::{Color, Point3, Vec3};

use crate::color::write_color;
use crate::material::{Dielectric, Lambertian, Metal, ScatterResult};
use crate::object::{Hittable, HittableList, Sphere};
use crate::util::random_double;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: usize) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 { return Color::new(0.0, 0.0, 0.0) }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(ScatterResult { scattered, attenuation }) = rec.mat.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_left_inner = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),  -0.4, material_left_inner)));
    world.add(Rc::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, material_right)));

    // Camera

    let cam = Camera::new();

    // Render

    /*let u = (180 as f64 + random_double()) / (image_width as f64 - 1.0);
    let v = (110 as f64 + random_double()) / (image_height as f64 - 1.0);
    let r = cam.get_ray(u, v);
    let pixel_color = ray_color(&r, &world, max_depth);
    dbg!(pixel_color);*/

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut stdout = std::io::stdout();

    for y in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", y);
        for x in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + random_double()) / (image_width as f64 - 1.0);
                let v = (y as f64 + random_double()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(&mut stdout, pixel_color, samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}
