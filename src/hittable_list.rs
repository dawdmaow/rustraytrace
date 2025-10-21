use crate::types::{HitRecord, Hittable, HittableList, Interval, Ray};

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn _with_object(object: Hittable) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn _clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: crate::types::Point3::zero(),
            normal: crate::types::Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: Box::new(crate::types::Material::Lambertian {
                albedo: crate::types::Color::zero(),
            }),
        };

        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
