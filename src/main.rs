use std::io::stderr;
use std::io::Write;
use std::rc::Rc;
use rand::prelude::*;
mod linear_algebra;
use linear_algebra::{Ray, Vec3};
mod geometry;
use geometry::{Hittable, HittableList, Sphere};
mod camera;
use camera::Camera;

type Colour = Vec3;
type Point = Vec3;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn ray_colour(ray: Ray, world: &Hittable) -> Colour {
    if let Some(record) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (record.normal + Colour::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
}

fn write_colour(pixel_colour: &Colour, samples_per_pixel: usize) {
    let r = pixel_colour.x()/(samples_per_pixel as f64);
    let g = pixel_colour.y()/(samples_per_pixel as f64);
    let b = pixel_colour.z()/(samples_per_pixel as f64);

    let red = (256.0*clamp(r,0.0,0.999)) as usize;
    let green = (256.0*clamp(g,0.0,0.999)) as usize;
    let blue = (256.0*clamp(b,0.0,0.999)) as usize;
    println!("{} {} {}", red, green, blue);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for row in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", row);
        stderr().flush().unwrap();
        let mut rng = rand::thread_rng();
        for col in 0..image_width {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel{
                let u = (col as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (row as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_colour += ray_colour(ray, &world);
            }
            write_colour(&pixel_colour, samples_per_pixel);
        }
    }
}
