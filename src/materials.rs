use crate::geometry::HitRecord;
use crate::linear_algebra::{random_unit_vector, reflect, refract, Ray};
use crate::random::rand;
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
        let scattered = Ray::new(
            hit_record.hit_point,
            reflected + self.fuzz * random_unit_vector(),
        );

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

pub struct Dielectric {
    refraction_index: f64,
    attenuation: Colour,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index,
            attenuation: Colour::new(1.0, 1.0, 1.0),
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r = ((1.0 - refraction_index) / (1.0 + refraction_index)).powf(2.0);
        r + (1.0 - r) * ((1.0 - cosine).powf(5.0))
    }
}

impl Material for Dielectric {
    fn scatter(&self, incident_ray: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = incident_ray.direction.unit_vector();

        let cosine = f64::min(1.0, -(unit_direction.dot(&hit_record.normal)));
        let sine = (1.0 - cosine * cosine).sqrt();
        let cannot_refract = refraction_ratio * sine > 1.0;
        let schlick_reflect = Dielectric::reflectance(cosine, refraction_ratio) > rand(0.0, 1.0);

        let direction = if cannot_refract || schlick_reflect {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };
        let scattered_ray = Ray::new(hit_record.hit_point, direction);

        Some(ScatteredRay {
            attenuation: self.attenuation,
            ray: scattered_ray,
        })
    }
}
