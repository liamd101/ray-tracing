use crate::{HitRecord, Ray};
use crate::{Hittable, Interval, Material, Point3, Vec3, AABB};

use std::sync::Arc;

#[derive(Clone)]
pub struct Cylinder {
    center: Vec3,
    radius: f32,
    height: f32,
    bbox: AABB,
    mat: Arc<dyn Material>,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f32, height: f32, mat: Arc<dyn Material>) -> Self {
        let bbox = AABB::new(
            Interval::new(center.x() - radius, center.x() + radius),
            Interval::new(center.y(), center.y() + height),
            Interval::new(center.z() - radius, center.z() + radius),
        );
        Self {
            center,
            radius,
            height,
            bbox,
            mat,
        }
    }

    fn hit_sides(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let dx = r.direction().x();
        let dz = r.direction().z();

        let ox = oc.x();
        let oz = oc.z();

        let a = dx * dx + dz * dz;
        let b = 2. * (ox * dx + oz * dz);
        let c = ox * ox + oz * oz - self.radius * self.radius;

        if a.abs() < f32::EPSILON {
            return false;
        }

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        let mut best_t = f32::INFINITY;
        let mut best_point = Vec3::default();

        for &t in &[t1, t2] {
            if !ray_t.surrounds(t) {
                continue;
            }

            // Calculate intersection point
            let hit_point = r.at(t);

            // Check if intersection is within cylinder height bounds (Y-axis)
            // Center is at bottom cap, so cylinder extends upward
            let y_min = self.center.y();
            let y_max = self.center.y() + self.height;

            if hit_point.y() >= y_min && hit_point.y() <= y_max && t < best_t {
                best_t = t;
                best_point = hit_point;
            }
        }

        // If we found a valid intersection
        if best_t < f32::INFINITY {
            rec.t = best_t;
            rec.p = best_point;

            // Calculate outward normal (pointing away from cylinder axis)
            let outward_normal = Vec3::new(
                (best_point.x() - self.center.x()) / self.radius,
                0.0,
                (best_point.z() - self.center.z()) / self.radius,
            );

            rec.set_face_normal(r, outward_normal);

            // Calculate UV coordinates for cylindrical surface
            self.get_sides_uv(
                &outward_normal,
                best_point.y(),
                &mut rec.u,
                &mut rec.v,
            );

            rec.mat = self.mat.clone();

            return true;
        }

        false
    }

    fn get_sides_uv(&self, outward_normal: &Vec3, hit_y: f32, u: &mut f32, v: &mut f32) {
        let theta = (-outward_normal.z()).atan2(outward_normal.x()) + std::f32::consts::PI;
        *u = theta / (2.0 * std::f32::consts::PI);
        *v = (hit_y - self.center.y()) / self.height;
    }

    fn hit_caps(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        if r.direction().y().abs() < f32::EPSILON {
            return false; // Ray is parallel to the caps
        }

        let y_min = self.center.y();
        let y_max = self.center.y() + self.height;

        for &(y_plane, normal_y) in &[(y_min, -1.0), (y_max, 1.0)] {
            let t = (y_plane - r.origin().y()) / r.direction().y();

            if !ray_t.surrounds(t) {
                continue;
            }

            let hit_point = r.at(t);

            let dx = hit_point.x() - self.center.x();
            let dz = hit_point.z() - self.center.z();
            let distance_from_center_sq = dx * dx + dz * dz;

            if distance_from_center_sq <= self.radius * self.radius {
                rec.t = t;
                rec.p = hit_point;
                let outward_normal = Vec3::new(0.0, normal_y, 0.0);
                rec.set_face_normal(r, outward_normal);
                self.get_cap_uv(&hit_point, &mut rec.u, &mut rec.v);
                rec.mat = self.mat.clone();
                return true;
            }
        }

        false
    }

    fn get_cap_uv(&self, hit_point: &Vec3, u: &mut f32, v: &mut f32) {
        let dx = hit_point.x() - self.center.x();
        let dy = hit_point.y() - self.center.y();
        *u = (dx / self.radius + 1.0) * 0.5;
        *v = (dy / self.radius + 1.0) * 0.5;
    }
}

impl Hittable for Cylinder {
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let mut closest_t = f32::INFINITY;
        let mut found_hit = false;
        let mut temp_rec = HitRecord::default();

        if self.hit_sides(r, ray_t, &mut temp_rec) {
            if temp_rec.t < closest_t && ray_t.surrounds(temp_rec.t) {
                closest_t = temp_rec.t;
                *rec = temp_rec.clone();
                found_hit = true;
            }
        }

        if self.hit_caps(r, ray_t, &mut temp_rec) {
            if temp_rec.t < closest_t && ray_t.surrounds(temp_rec.t) {
                *rec = temp_rec.clone();
                found_hit = true;
            }
        }

        found_hit
    }
}
