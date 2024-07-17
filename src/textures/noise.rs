use crate::Texture;

pub struct Perlin {
    point_count: usize,
    rand_float: Vec<f32>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count = 256;
        let rand_float = (0..point_count).map(|_| rand::random::<f32>()).collect();
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            point_count,
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &crate::Point3) -> f32 {
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;
        self.rand_float
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = (0..256).collect::<Vec<i32>>();
        Self::permute(&mut p, 256);
        p
    }

    fn permute(p: &mut [i32], n: i32) {
        for i in (1..n).rev() {
            let target = (rand::random::<f32>() * (i + 1) as f32) as usize;
            p.swap(i as usize, target);
        }
    }
}

pub struct PerlinNoise {
    noise: Perlin,
}
impl PerlinNoise {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}
impl Texture for PerlinNoise {
    fn value(&self, u: f32, v: f32, p: &crate::Point3) -> crate::Color {
        crate::Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
