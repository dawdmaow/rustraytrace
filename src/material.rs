use crate::types::{Color, HitRecord, Material, Ray};
use crate::vec3::{dot, random_unit_vector, reflect, refract, unit_vector};
use rand::Rng;

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = *albedo;
                true
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(unit_vector(r_in.direction), rec.normal);
                let direction = unit_vector(reflected) + (*fuzz * random_unit_vector());

                *scattered = Ray::new(rec.p, direction);
                *attenuation = *albedo;

                dot(scattered.direction, rec.normal) > 0.0
            }
            Material::Dielectric { refraction_index } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let ri = if rec.front_face {
                    1.0 / refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = unit_vector(r_in.direction);
                let cos_theta = (-dot(unit_direction, rec.normal)).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = ri * sin_theta > 1.0;
                let mut rng = rand::rng();

                let reflectance = |cosine: f64, ri: f64| -> f64 {
                    let r0 = (1.0 - ri) / (1.0 + ri);
                    let r0 = r0 * r0;
                    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
                };

                let direction =
                    if cannot_refract || reflectance(cos_theta, ri) > rng.random_range(0.0..1.0) {
                        reflect(unit_direction, rec.normal)
                    } else {
                        refract(unit_direction, rec.normal, ri)
                    };

                *scattered = Ray::new(rec.p, direction);
                true
            }
        }
    }
}
