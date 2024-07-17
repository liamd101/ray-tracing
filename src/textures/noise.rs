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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c: [[[f32; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.rand_float[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f32 * u + (1.0 - i as f32) * (1.0 - u))
                        * (j as f32 * v + (1.0 - j as f32) * (1.0 - v))
                        * (k as f32 * w + (1.0 - k as f32) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
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
