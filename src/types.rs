#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Box<Material>,
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}

#[derive(Debug)]
pub enum Hittable {
    Sphere(Sphere),
    // Plane(Plane),
    // Triangle(Triangle),
}

// #[derive(Debug)]
// pub struct Plane {
//     pub point: crate::types::Point3,
//     pub normal: crate::types::Vec3,
//     pub mat: Box<crate::types::Material>,
// }

// #[derive(Debug)]
// pub struct Triangle {
//     pub v0: crate::types::Point3,
//     pub v1: crate::types::Point3,
//     pub v2: crate::types::Point3,
//     pub mat: Box<crate::types::Material>,
// }

#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Hittable>,
}

#[derive(Debug)]
pub struct Sphere {
    pub center: crate::types::Point3,
    pub radius: f64,
    pub mat: Box<crate::types::Material>,
}
