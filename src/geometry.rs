use crate::linear_algebra::{Ray, Vec3};
use std::rc::Rc;

type Point = Vec3;

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct HitRecord {
    pub t: f64,
    pub hit_point: Point,
    pub normal: Vec3,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(t: f64, ray: Ray, outward_normal: Vec3) -> Self {
        let hit_point = ray.at(t);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        return HitRecord {
            t: t,
            hit_point: hit_point,
            normal: normal,
            front_face: front_face,
        };
    }
}

#[derive(Debug)]
pub struct Sphere {
    centre: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Point, radius: f64) -> Self {
        Sphere {
            centre: centre,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant <= 0.0 {
            return None;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;

        if root < t_min || root > t_max {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - self.centre) / self.radius;

        Some(HitRecord::new(root, ray, outward_normal))
    }
}

pub struct HittableList{
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList{
    pub fn new() -> Self{
        HittableList{objects: Vec::<Rc<dyn Hittable>>::new()}
    }

    pub fn add(&mut self, item: Rc<dyn Hittable>){
        self.objects.push(item);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let mut closest_hit = None;
        let mut min_t = t_max;

        for object in &self.objects{
            if let Some(hit_record) = object.hit(ray, t_min, t_max) {
                if hit_record.t < min_t{
                    min_t = hit_record.t;
                    closest_hit = Some(hit_record);
                }
            }
        }

        return closest_hit;
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    mod sphere_tests{
        use super::*;
        #[test]
        fn hit() {
            let sphere = Sphere::new(Point::new(0.0, 0.0, -2.0), 1.0);
            let hit_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
            let miss_ray = Ray::new(hit_ray.origin, -hit_ray.origin);
            let internal_ray = Ray::new(sphere.centre, hit_ray.direction);

            assert!(sphere.hit(miss_ray, 0.0, 100.0).is_none());
            
            let hit_record = sphere.hit(hit_ray, 0.0, 100.0).unwrap();
            let internal_hit_record = sphere.hit(internal_ray, 0.0, 100.0).unwrap();

            assert!(approx_eq(hit_record.t, 1.0, f64::EPSILON));
            assert!(approx_eq(internal_hit_record.t, 1.0, f64::EPSILON));

            assert!(hit_record.front_face);
            assert!(!internal_hit_record.front_face);
        }
    }

    mod hittable_list_tests{
        use super::*;

        #[test]
        fn hit() {
            let mut list = HittableList::new();
            let sphere1 = Sphere::new(Point::new(0.0, 0.0, -2.0), 1.0);
            let sphere2 = Sphere::new(Point::new(0.0, 10.0, 0.0), 1.0);
            let sphere3 = Sphere::new(Point::new(0.0, 0.0, -20.0), 1.0);
            list.add(Rc::new(sphere1));
            list.add(Rc::new(sphere2));
            list.add(Rc::new(sphere3));

            let miss_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
            let ray1 = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
            let ray2 = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 10.0, 0.0).unit_vector());

            let miss = list.hit(miss_ray, 0.0, 100.0);
            let hit1 = list.hit(ray1, 0.0, 100.0);
            let hit2 = list.hit(ray2, 0.0, 100.0);

            assert!(miss.is_none());
            assert!(approx_eq(hit1.unwrap().t, 1.0, f64::EPSILON));
            assert!(approx_eq(hit2.unwrap().t, 9.0, f64::EPSILON));
        }
    }

    fn approx_eq(x: f64, y: f64, tolerance: f64) -> bool {
        (x - y).abs() < tolerance
    }
}