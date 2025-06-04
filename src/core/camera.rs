use exr::prelude::*;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use crate::utils::{degrees_to_radians, random_double, random_double_range, INFINITY};
use crate::{vec3, Color, HitRecord, Hittable, Interval, Point3, Ray, Vec3};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub background: Color,
    pub vfov: f32,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
    pub file_path: String,
    image_height: usize,
    pixel_samples_scale: f32,
    /// Square root of number of samples per pixel
    sqrt_spp: usize,
    /// 1 / sqrt_spp
    recip_sqrt_spp: f32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,
            background: Color::new(0.0, 0.0, 0.0),
            vfov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            file_path: "image.ppm".into(),
            image_height: 0,
            pixel_samples_scale: 1.0 / 10.0,
            sqrt_spp: 0,
            recip_sqrt_spp: 0.,
            center: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let pixels = self.render_pixels_parallel(world);
        self.write_image(pixels);
    }

    pub fn render_pixels_parallel(&self, world: &dyn Hittable) -> Vec<(usize, usize, Vec3)> {
        let total_pixels = (self.image_height * self.image_width) as u64;
        (0..self.image_height)
            .into_par_iter()
            .flat_map(|y| {
                (0..self.image_width).into_par_iter().map(move |x| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for s_j in 0..self.sqrt_spp {
                        for s_i in 0..self.sqrt_spp {
                            let ray = self.get_ray(x, y, s_i, s_j);
                            pixel_color += self.ray_color(&ray, self.max_depth, world);
                        }
                    }
                    (x, y, self.pixel_samples_scale * pixel_color)
                })
            })
            .progress_count(total_pixels)
            .collect()
    }

    fn write_image(&self, pixels: Vec<(usize, usize, Vec3)>) {
        let mut image: Vec<f32> = Vec::with_capacity(self.image_height * self.image_width * 3);
        let mut pixel_map = vec![vec![[0.0f32; 3]; self.image_width]; self.image_height];

        for (x, y, col) in pixels {
            pixel_map[y][x] = [col.x(), col.y(), col.z()];
        }

        for row in pixel_map {
            for pixel in row {
                image.extend(&pixel);
            }
        }

        Image::from_channels(
            (self.image_width, self.image_height),
            SpecificChannels::rgb(|Vec2(x, y)| {
                let idx = (y * self.image_width + x) * 3;
                (image[idx], image[idx + 1], image[idx + 2])
            }),
        )
        .write()
        .to_file(&self.file_path)
        .unwrap();
    }

    fn get_ray(&self, i: usize, j: usize, s_i: usize, s_j: usize) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
            + (((i as f32) + offset.x()) * self.pixel_delta_u)
            + (((j as f32) + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle > 0.0 {
            self.defocus_disk_sample()
        } else {
            self.look_from
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y())
    }

    fn sample_square_stratified(&self, s_i: usize, s_j: usize) -> Vec3 {
        let px = ((s_i as f32 + random_double()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f32 + random_double()) * self.recip_sqrt_spp) - 0.5;
        Vec3::new(px, py, 0.)
    }

    fn initialize(&mut self) {
        let aspect_ratio = self.aspect_ratio;
        let image_width = self.image_width;

        let image_height = (image_width as f32 / aspect_ratio) as usize;
        self.image_height = if image_height == 0 { 1 } else { image_height };

        self.sqrt_spp = (f32::sqrt(self.samples_per_pixel as f32)) as usize;
        self.pixel_samples_scale = 1.0 / (self.sqrt_spp * self.sqrt_spp) as f32;
        self.recip_sqrt_spp = 1.0 / self.sqrt_spp as f32;

        self.center = self.look_from;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        self.w = vec3::unit_vector(self.look_from - self.look_at);
        self.u = vec3::unit_vector(vec3::cross(self.vup, self.w));
        self.v = vec3::cross(self.w, self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v = viewport_v / (image_height as f32);
        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
    }

    fn ray_color(&self, r: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec: HitRecord = Default::default();
        if !world.hit(r, &mut Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }

        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let mut pdf_value = 0.;
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);

        if !rec
            .mat
            .scatter(r, &rec, &mut attenuation, &mut scattered, &mut pdf_value)
        {
            return color_from_emission;
        }

        let on_light = Point3::new(
            random_double_range(213., 343.),
            554.,
            random_double_range(227., 332.),
        );
        let to_light = on_light - rec.p;
        let distance_squared = to_light.length_squared();
        let to_light = vec3::unit_vector(to_light);

        if vec3::dot(to_light, rec.normal) < 0. {
            return color_from_emission;
        }
        let light_area = (343. - 213.) * (332. - 227.);
        let light_cosine = to_light.y().abs();
        if light_cosine < 0.000001 {
            return color_from_emission;
        }

        let pdf_value = distance_squared / (light_cosine * light_area);
        scattered = Ray::new(rec.p, to_light, r.time());

        let scattering_pdf = rec.mat.scattering_pdf(r, &rec, &scattered);

        let color_from_scatter =
            scattering_pdf * attenuation * self.ray_color(&scattered, depth - 1, world) / pdf_value;
        color_from_emission + color_from_scatter
    }
}
