use ray_tracing::{
    utils, BvhNode, Camera, Checkerboard, Color, Dielectric, HittableList, Lambertian, Material,
    Metal, Point3, SolidColor, Sphere, Vec3,
};

use std::rc::Rc;

fn bouncing_spheres() {
    let mut world: HittableList = HittableList::new();

    let even = Rc::new(SolidColor::from_rgb(0.2, 0.3, 0.1));
    let odd = Rc::new(SolidColor::from_rgb(0.9, 0.9, 0.9));
    let checker = Checkerboard::new(even, odd, 0.32);
    let material_ground = Lambertian::with_texture(Rc::new(checker));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(material_ground),
    )));

    for a in -2..2 {
        for b in -2..2 {
            let choose_mat = utils::random_double();
            let center = Point3::new(
                a as f32 + 0.9 * utils::random_double(),
                0.2,
                b as f32 + 0.9 * utils::random_double(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material>;
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Box::new(Lambertian::new(albedo));
                    let center2 =
                        center + Point3::new(0.0, utils::random_double_range(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = utils::random_double_range(0.0, 0.5);
                    sphere_material = Box::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::stationary(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Box::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::stationary(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.0 / 1.5);
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        0.5,
        Box::new(material1),
    )));

    let material2 = Dielectric::new(1.5);
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(material2),
    )));

    let material3 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(material3),
    )));

    let material4 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Rc::new(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(material4),
    )));

    let world = BvhNode::from_list(world);

    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 100;
    cam.max_depth = 25;

    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

fn checkered_spheres() {
    let mut world: HittableList = HittableList::new();
    let checker = Checkerboard::new(
        Rc::new(SolidColor::from_rgb(0.2, 0.3, 0.1)),
        Rc::new(SolidColor::from_rgb(0.9, 0.9, 0.9)),
        0.32,
    );
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Box::new(Lambertian::with_texture(Rc::new(checker.clone()))),
    )));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Box::new(Lambertian::with_texture(Rc::new(checker))),
    )));

    let world = BvhNode::from_list(world);

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    checkered_spheres();
}
