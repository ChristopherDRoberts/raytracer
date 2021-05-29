use std::io::stderr;
use std::io::Write;
use std::rc::Rc;
mod random;
use random::rand;
mod linear_algebra;
use linear_algebra::{Ray, Vec3, random_unit_vector, random_vector};
mod geometry;
use geometry::{Hittable, HittableList, Sphere};
mod camera;
use camera::Camera;
mod materials;
use materials::{Lambertian, Metal, Dielectric};
use std::f64::consts::PI;

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

fn ray_colour(ray: Ray, world: &dyn Hittable, depth: usize) -> Colour {
    if depth == 0{
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(ray, 0.001, f64::INFINITY) {

        if let Some(scattered_ray) = record.material.scatter(&ray, &record){
            return scattered_ray.attenuation * ray_colour(scattered_ray.ray, world, depth - 1);
        }

        return Colour::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn write_colour(pixel_colour: &Colour, samples_per_pixel: usize) {
    let r = (pixel_colour.x()/(samples_per_pixel as f64)).sqrt();
    let g = (pixel_colour.y()/(samples_per_pixel as f64)).sqrt();
    let b = (pixel_colour.z()/(samples_per_pixel as f64)).sqrt();

    let red = (256.0*clamp(r,0.0,0.999)) as usize;
    let green = (256.0*clamp(g,0.0,0.999)) as usize;
    let blue = (256.0*clamp(b,0.0,0.999)) as usize;
    println!("{} {} {}", red, green, blue);
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground.clone())));

    let offset_point = Vec3::new(4.0, 0.2, 0.0);
    let glass_material = Rc::new(Dielectric::new(1.5));

    for a in -11..11{
        for b in -11..11{
            let choose_mat = rand(0.0,1.0);
            let centre = Vec3::new(a as f64 + 0.9*rand(0.0,1.0), 0.2, b as f64 + 0.9*rand(0.0,1.0));

            if (centre - offset_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_vector(0.0, 1.0) * random_vector(0.0, 1.0);
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(centre, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = random_vector(0.5, 1.0);
                    let fuzz = rand(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(centre, 0.2, sphere_material)));
                } else {
                    let sphere_material = glass_material.clone();
                    world.add(Rc::new(Sphere::new(centre, 0.2, sphere_material)));
                }
            }
        }
    }

    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, glass_material.clone())));

    let brown_material = Rc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, brown_material)));

    let shiny_steel_material = Rc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, shiny_steel_material)));
    

    // Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vec_up = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(look_from, look_at, vec_up, 20.0, aspect_ratio, aperture, focus_dist);

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for row in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", row);
        stderr().flush().unwrap();
        for col in 0..image_width {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel{
                let u = (col as f64 + rand(0.0, 1.0)) / (image_width - 1) as f64;
                let v = (row as f64 + rand(0.0, 1.0)) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_colour += ray_colour(ray, &world, max_depth);
            }
            write_colour(&pixel_colour, samples_per_pixel);
        }
    }
}
