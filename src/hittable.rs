use crate::types::{HitRecord, Hittable, Interval, Ray, Sphere, Vec3};

// TODO: use plane and triangle in rendered image

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = crate::vec3::dot(ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Hittable {
    pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(r, ray_t, rec),
            // Hittable::Plane(plane) => plane.hit(r, ray_t, rec),
            // Hittable::Triangle(triangle) => triangle.hit(r, ray_t, rec),
        }
    }
}

impl Sphere {
    pub fn new(
        center: crate::types::Point3,
        radius: f64,
        mat: Box<crate::types::Material>,
    ) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = crate::vec3::dot(r.direction, r.direction);
        let half_b = crate::vec3::dot(oc, r.direction);
        let c = crate::vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}

// impl Plane {
//     pub fn new(
//         point: crate::types::Point3,
//         normal: crate::types::Vec3,
//         mat: Box<crate::types::Material>,
//     ) -> Self {
//         Self { point, normal, mat }
//     }

//     pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
//         let denom = crate::vec3::dot(self.normal, r.direction);
//         if denom.abs() < 1e-8 {
//             return false;
//         }

//         let t = crate::vec3::dot(self.point - r.origin, self.normal) / denom;
//         if !ray_t.surrounds(t) {
//             return false;
//         }

//         rec.t = t;
//         rec.p = r.at(rec.t);
//         rec.set_face_normal(r, self.normal);
//         rec.mat = self.mat.clone();

//         true
//     }
// }

// impl Triangle {
//     pub fn new(
//         v0: crate::types::Point3,
//         v1: crate::types::Point3,
//         v2: crate::types::Point3,
//         mat: Box<crate::types::Material>,
//     ) -> Self {
//         Self { v0, v1, v2, mat }
//     }

//     pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
//         let edge1 = self.v1 - self.v0;
//         let edge2 = self.v2 - self.v0;
//         let h = crate::vec3::cross(r.direction, edge2);
//         let a = crate::vec3::dot(edge1, h);

//         if a.abs() < 1e-8 {
//             return false;
//         }

//         let f = 1.0 / a;
//         let s = r.origin - self.v0;
//         let u = f * crate::vec3::dot(s, h);

//         if u < 0.0 || u > 1.0 {
//             return false;
//         }

//         let q = crate::vec3::cross(s, edge1);
//         let v = f * crate::vec3::dot(r.direction, q);

//         if v < 0.0 || u + v > 1.0 {
//             return false;
//         }

//         let t = f * crate::vec3::dot(edge2, q);
//         if !ray_t.surrounds(t) {
//             return false;
//         }

//         rec.t = t;
//         rec.p = r.at(rec.t);
//         let normal = crate::vec3::cross(edge1, edge2);
//         rec.set_face_normal(r, normal);
//         rec.mat = self.mat.clone();

//         true
//     }
// }
