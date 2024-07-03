use crate::{vec3, HitRecord, Hittable, Interval, Material, Point3, Ray, Vec3, AABB};

pub struct Quadrilateral {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: Box<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f32,
}

impl Quadrilateral {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Box<dyn Material>) -> Self {
        let bbox_diagonal_1 = AABB::around_points(q, q + u + v);
        let bbox_diagonal_2 = AABB::around_points(q + u, q + v);
        let bbox = AABB::around_boxes(&bbox_diagonal_1, &bbox_diagonal_2);
        let n = vec3::cross(u, v);
        let normal = vec3::unit_vector(n);
        let d = vec3::dot(normal, q);
        let w = n / vec3::dot(n, n);

        Self {
            q,
            u,
            v,
            w,
            material,
            bbox,
            normal,
            d,
        }
    }

    fn is_interior(&self, alpha: f32, beta: f32, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return false;
        }

        rec.u = alpha;
        rec.v = beta;
        true
    }
}

impl Hittable for Quadrilateral {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let denom = vec3::dot(self.normal, r.direction());

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - vec3::dot(self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = vec3::dot(self.w, vec3::cross(planar_hitpt_vector, self.v));
        let beta = vec3::dot(self.w, vec3::cross(self.u, planar_hitpt_vector));

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.material.clone();
        rec.normal = self.normal;

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
