use crate::Texture;

pub struct Perlin {
    rand_vec: Vec<crate::Vec3>,
    rand_float: Vec<f32>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count = 256;
        let rand_vec = (0..point_count)
            .map(|_| crate::vec3::unit_vector(crate::Vec3::random_range(-1.0, 1.0)))
            .collect();
        let rand_float = (0..point_count).map(|_| rand::random::<f32>()).collect();
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            rand_vec,
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
        let mut c: [[[crate::Vec3; 2]; 2]; 2] = [[[crate::Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    fn turb(&self, p: &crate::Point3, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }

    fn trilinear_interp(c: [[[crate::Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = crate::Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                        * crate::vec3::dot(c[i][j][k], weight_v);
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
    scale: f32,
}
impl PerlinNoise {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}
impl Texture for PerlinNoise {
    fn value(&self, u: f32, v: f32, p: &crate::Point3) -> crate::Color {
        crate::Color::new(1.0, 1.0, 1.0) * self.noise.turb(&p, 7)
    }
}
