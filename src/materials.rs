use crate::geometry::HitRecord;
use crate::linear_algebra::{random_unit_vector, reflect, Ray};
use crate::Colour;

pub struct ScatteredRay {
    pub attenuation: Colour,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay>;
}

pub struct EmptyMaterial;

impl Material for EmptyMaterial {
    fn scatter(&self, _incident_ray: &Ray, _hit_record: &HitRecord) -> Option<ScatteredRay> {
        None
    }
}

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _incident_ray: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered_ray = ScatteredRay {
            attenuation: self.albedo,
            ray: Ray::new(hit_record.hit_point, scatter_direction),
        };

        Some(scattered_ray)
    }
}

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let reflected = reflect(incident_ray.direction, hit_record.normal);
        let scattered = Ray::new(hit_record.hit_point, reflected + self.fuzz * random_unit_vector());

        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            return Some(ScatteredRay {
                attenuation: self.albedo,
                ray: scattered,
            });
        } else {
            return None;
        }
    }
}
