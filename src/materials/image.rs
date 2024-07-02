use crate::{Color, Texture, Vec3};
use stb_image::image::{self, Image};

pub struct ImageTexture {
    image: Image<f32>,
}

impl ImageTexture {
    pub fn from_file(path: &str) -> Self {
        let image = match image::load(path) {
            image::LoadResult::Error(e) => panic!("Error loading image: {}", e),
            image::LoadResult::ImageU8(_) => panic!("Expected f32 image, got u8"),
            image::LoadResult::ImageF32(image) => image,
        };
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        if self.image.height == 0 {
            return Vec3::new(0.0, 1.0, 1.0);
        }
        let u = u.min(1.0).max(0.0);
        let v = 1.0 - v.min(1.0).max(0.0);

        let i = (u * self.image.width as f32) as i32;
        let j = (v * self.image.height as f32) as i32;
        let color_scale = 1.0 / 255.0;
        let pixel = pixel_at(&self.image, i, j);
        pixel * color_scale
    }
}

fn pixel_at(image: &Image<f32>, i: i32, j: i32) -> Vec3 {
    let i = i.min(image.width as i32 - 1).max(0);
    let j = j.min(image.height as i32 - 1).max(0);
    let pixel = &image.data[(j * image.width as i32 + i) as usize];
    todo!()
}
