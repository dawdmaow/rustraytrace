mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod types;
mod vec3;

use camera::Camera;
use rand::Rng;
use types::{Color, Hittable, HittableList, Material, Point3, Sphere, Vec3};

fn main() {
    let mut world = HittableList::new();

    world.add(Hittable::Sphere(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Material::Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    )));

    // Random small spheres
    let mut rng = rand::rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.random_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Box::new(Material::Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    Box::new(Material::Metal { albedo, fuzz })
                } else {
                    // glass
                    Box::new(Material::Dielectric {
                        refraction_index: 1.5,
                    })
                };
                world.add(Hittable::Sphere(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    world.add(Hittable::Sphere(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Material::Dielectric {
            refraction_index: 1.5,
        }),
    )));

    world.add(Hittable::Sphere(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Material::Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    )));

    world.add(Hittable::Sphere(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 10;
    cam.max_depth = 20;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
