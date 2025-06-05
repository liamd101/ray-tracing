use crate::{
    utils, vec3, HitRecord, Hittable, Interval, Material, NoneMaterial, Point3, Ray, Vec3, AABB,
    ONB,
};

use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Arc<dyn Material>,
    bbox: AABB,
    is_moving: bool,
    center_vec: Vec3,
}
impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Point3::default(),
            radius: 0.0,
            mat: Arc::new(NoneMaterial),
            bbox: AABB::default(),
            is_moving: false,
            center_vec: Vec3::default(),
        }
    }
}

impl Sphere {
    pub fn stationary(center: Point3, radius: f32, mat: Arc<dyn Material>) -> Self {
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

    pub fn moving(center1: Point3, center2: Point3, radius: f32, mat: Arc<dyn Material>) -> Self {
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

    fn get_sphere_uv(p: Vec3, u: &mut f32, v: &mut f32) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f32::consts::PI;
        *u = phi / (2.0 * std::f32::consts::PI);
        *v = theta / std::f32::consts::PI;
    }

    fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3 {
        let r1 = utils::random_double();
        let r2 = utils::random_double();
        let z = 1. + r2 * (
            (1. - (radius * radius / distance_squared)).sqrt() - 1.
            );

        let phi = 2. * std::f32::consts::PI * r1;
        let x = phi.cos() * (1. - z * z).sqrt();
        let y = phi.sin() * (1. - z * z).sqrt();

        Vec3::new(x, y, z)
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
        Self::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let direction = self.sphere_center(0.) - origin;
        let distance_squared = direction.length_squared();
        let uvw = ONB::new(direction);
        uvw.transform(Sphere::random_to_sphere(self.radius, distance_squared))
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f32 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new(origin, direction, 0.),
            &mut Interval::new(0.001, f32::INFINITY),
            &mut rec,
        ) {
            return 0.;
        }

        let dist_squared = (self.sphere_center(0.) - origin).length_squared();
        let cos_theta_max = (1. - (self.radius * self.radius/dist_squared)).sqrt();
        let solid_angle = 2. * std::f32::consts::PI * (1. - cos_theta_max);

        1. / solid_angle
    }
}
