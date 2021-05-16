use std::rc::Rc;
use std::io::stderr;
use std::io::Write;
mod linear_algebra;
use linear_algebra::{Ray, Vec3};
mod geometry;
use geometry::{Sphere, HittableList, Hittable};

type Colour = Vec3;
type Point = Vec3;

/* fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
} */

/* fn ray_colour(ray: &Ray) -> Colour {
    let t = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Colour::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    //(1.0-t)*Colour::new(1.0, 1.0, 1.0)+t*Colour::new(0.0, 0.0, 0.0) // B&W gradient
} */

fn ray_colour(ray: Ray, world: &Hittable) -> Colour{
    if let Some(record) = world.hit(ray, 0.0, f64::INFINITY){
        return 0.5 * (record.normal + Colour::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
}

fn write_colour(pixel_colour: &Colour) {
    let red = (255.999 * pixel_colour.x()) as usize;
    let green = (255.999 * pixel_colour.y()) as usize;
    let blue = (255.999 * pixel_colour.z()) as usize;
    println!("{} {} {}", red, green, blue);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for row in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", row);
        stderr().flush().unwrap();
        for col in 0..image_width {
            let u = (col as f64) / (image_width - 1) as f64;
            let v = (row) as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_colour = ray_colour(ray, &world);
            write_colour(&pixel_colour);
        }
    }
}
