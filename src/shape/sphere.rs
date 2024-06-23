use crate::{vec3, HitRecord, Hittable, Interval, Material, NoneMaterial, Point3, Ray, Vec3, AABB};

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Box<dyn Material>,
    bbox: AABB,
    is_moving: bool,
    center_vec: Vec3,
}
impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Point3::default(),
            radius: 0.0,
            mat: Box::new(NoneMaterial),
            bbox: AABB::default(),
            is_moving: false,
            center_vec: Vec3::default(),
        }
    }
}

impl Sphere {
    pub fn stationary(center: Point3, radius: f32, mat: Box<dyn Material>) -> Self {
        let rvec = Vec3([radius, radius, radius]);
        let bbox = AABB::around_points(center - rvec, center + rvec);
        Sphere {
            center,
            radius,
            mat,
            bbox,
            is_moving: false,
            ..Default::default()
        }
    }

    pub fn moving(center1: Point3, center2: Point3, radius: f32, mat: Box<dyn Material>) -> Self {
        let rvec = Vec3([radius, radius, radius]);
        let bbox = AABB::around_points(center1 - rvec, center1 + rvec);
        let center_vec = center2 - center1;
        Sphere {
            center: center1,
            radius,
            mat,
            bbox,
            is_moving: true,
            center_vec,
        }
    }

    pub fn sphere_center(&self, time: f32) -> Point3 {
        self.center + time * self.center_vec
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            self.sphere_center(r.time())
        } else {
            self.center
        };
        let oc = center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (h * h) - (a * c);
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
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

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
