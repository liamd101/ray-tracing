// src/materials/cook_torrance.rs
use super::material::{Material, ScatterRecord};
use crate::{utils, vec3, Color, CosinePdf, HitRecord, Ray, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct Glossy {
    albedo: Color,
    roughness: f32,
    metallic: f32,
    f0: Color, // Base reflectance at normal incidence
}

impl Glossy {
    pub fn new(albedo: Color, roughness: f32, metallic: f32) -> Self {
        // For dielectrics, F0 is typically around 0.04
        // For metals, F0 is the albedo color
        let f0 = Color::new(0.04, 0.04, 0.04) * (1.0 - metallic) + albedo * metallic;

        Self {
            albedo,
            roughness: roughness.max(0.001), // Avoid division by zero
            metallic,
            f0,
        }
    }

    // GGX Normal Distribution Function
    fn distribution_ggx(&self, n: Vec3, h: Vec3) -> f32 {
        let a = self.roughness * self.roughness;
        let a2 = a * a;
        let n_dot_h = vec3::dot(n, h).max(0.0);
        let n_dot_h2 = n_dot_h * n_dot_h;

        let num = a2;
        let denom = n_dot_h2 * (a2 - 1.0) + 1.0;
        let denom = std::f32::consts::PI * denom * denom;

        num / denom
    }

    // Smith's method for geometry function
    fn geometry_schlick_ggx(&self, n_dot_v: f32) -> f32 {
        let r = self.roughness + 1.0;
        let k = (r * r) / 8.0;

        let num = n_dot_v;
        let denom = n_dot_v * (1.0 - k) + k;

        num / denom
    }

    fn geometry_smith(&self, n: Vec3, v: Vec3, l: Vec3) -> f32 {
        let n_dot_v = vec3::dot(n, v).max(0.0);
        let n_dot_l = vec3::dot(n, l).max(0.0);
        let ggx2 = self.geometry_schlick_ggx(n_dot_v);
        let ggx1 = self.geometry_schlick_ggx(n_dot_l);

        ggx1 * ggx2
    }

    // Fresnel-Schlick approximation
    fn fresnel_schlick(&self, cos_theta: f32, f0: Color) -> Color {
        let cos_theta = cos_theta.max(0.0).min(1.0);
        f0 + (Color::new(1.0, 1.0, 1.0) - f0) * (1.0 - cos_theta).powf(5.0)
    }

    // Sample the GGX distribution to get a half vector
    fn sample_ggx(&self, n: Vec3) -> Vec3 {
        let r1 = utils::random_double();
        let r2 = utils::random_double();

        let a2 = self.roughness * self.roughness;

        // Sample theta (polar angle)
        let cos_theta = ((1.0 - r1) / (1.0 + (a2 - 1.0) * r1)).sqrt();
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Sample phi (azimuthal angle)
        let phi = 2.0 * std::f32::consts::PI * r2;

        // Convert to Cartesian coordinates in tangent space
        let h_tangent = Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta);

        // Transform from tangent space to world space
        // We need to build an orthonormal basis around the normal
        let up = if n.z().abs() < 0.999 {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let tangent = vec3::unit_vector(vec3::cross(up, n));
        let bitangent = vec3::cross(n, tangent);

        tangent * h_tangent.x() + bitangent * h_tangent.y() + n * h_tangent.z()
    }

    // Evaluate the full Cook-Torrance BRDF
    fn evaluate_brdf(&self, n: Vec3, v: Vec3, l: Vec3) -> Color {
        let h = vec3::unit_vector(v + l);

        let n_dot_v = vec3::dot(n, v).max(0.0);
        let n_dot_l = vec3::dot(n, l).max(0.0);
        let h_dot_v = vec3::dot(h, v).max(0.0);

        if n_dot_v <= 0.0 || n_dot_l <= 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let d = self.distribution_ggx(n, h);
        let g = self.geometry_smith(n, v, l);
        let f = self.fresnel_schlick(h_dot_v, self.f0);

        // Cook-Torrance specular BRDF
        let specular = (f * d * g) / (4.0 * n_dot_v * n_dot_l);

        // Lambertian diffuse BRDF
        let kd = (Color::new(1.0, 1.0, 1.0) - f) * (1.0 - self.metallic);
        let diffuse = kd * self.albedo / std::f32::consts::PI;

        diffuse + specular
    }

    // PDF for GGX sampling
    fn ggx_pdf(&self, n: Vec3, h: Vec3, v: Vec3) -> f32 {
        let d = self.distribution_ggx(n, h);
        let n_dot_h = vec3::dot(n, h).max(0.0);
        let h_dot_v = vec3::dot(h, v).max(0.0);

        if h_dot_v <= 0.0 {
            0.0
        } else {
            (d * n_dot_h) / (4.0 * h_dot_v)
        }
    }
}

impl Material for Glossy {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let v = -vec3::unit_vector(r_in.direction());
        let n = rec.normal;

        // For path tracing, we need to choose between diffuse and specular sampling
        // We'll use a simple heuristic based on Fresnel reflectance
        let f0_avg = (self.f0.x() + self.f0.y() + self.f0.z()) / 3.0;
        let fresnel_avg = self
            .fresnel_schlick(vec3::dot(n, v), Color::new(f0_avg, f0_avg, f0_avg))
            .x();

        // Probability of sampling specular vs diffuse
        let spec_prob = 0.25 + 0.75 * fresnel_avg;

        if utils::random_double() < spec_prob {
            // Sample the specular lobe using GGX distribution
            let h = self.sample_ggx(n);
            let l = vec3::reflect(-v, h);

            // Check if the reflected ray is above the surface
            if vec3::dot(l, n) <= 0.0 {
                return false;
            }

            srec.attenuation =
                self.evaluate_brdf(n, v, l) * vec3::dot(l, n) / (self.ggx_pdf(n, h, v) * spec_prob);
            srec.skip_pdf = true;
            srec.skip_pdf_ray = Ray::new(rec.p, l, r_in.time());
        } else {
            // Sample the diffuse lobe using cosine-weighted hemisphere sampling
            srec.attenuation = self.evaluate_brdf(n, v, vec3::random_cosine_direction())
                / ((1.0 - spec_prob) / std::f32::consts::PI);
            srec.pdf = Arc::new(CosinePdf::new(n));
            srec.skip_pdf = false;
        }

        true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        let v = -vec3::unit_vector(r_in.direction());
        let l = vec3::unit_vector(scattered.direction());
        let n = rec.normal;
        let h = vec3::unit_vector(v + l);

        let cos_theta = vec3::dot(n, l);
        if cos_theta <= 0.0 {
            return 0.0;
        }

        // Combined PDF: weighted mixture of diffuse and specular
        let f0_avg = (self.f0.x() + self.f0.y() + self.f0.z()) / 3.0;
        let fresnel_avg = self
            .fresnel_schlick(vec3::dot(n, v), Color::new(f0_avg, f0_avg, f0_avg))
            .x();
        let spec_prob = 0.25 + 0.75 * fresnel_avg;

        let diffuse_pdf = cos_theta / std::f32::consts::PI;
        let specular_pdf = self.ggx_pdf(n, h, v);

        (1.0 - spec_prob) * diffuse_pdf + spec_prob * specular_pdf
    }
}
