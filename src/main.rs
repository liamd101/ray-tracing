use ray_tracing::{
    new_box, utils, BvhNode, Camera, Checkerboard, Color, Dielectric, DiffuseLight, HittableList,
    Lambertian, Material, Metal, Point3, Quadrilateral as Quad, RotateY, SolidColor, Sphere,
    Translate, Vec3, PerlinNoise
};

use clap::{Parser, Subcommand};

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
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 25;
    cam.background = Color::new(0.70, 0.80, 1.00);

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
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn simple_light() {
    let mut world: HittableList = HittableList::new();
    let checker = Checkerboard::new(
        Rc::new(SolidColor::from_rgb(0.2, 0.3, 0.1)),
        Rc::new(SolidColor::from_rgb(0.9, 0.9, 0.9)),
        0.32,
    );
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::with_texture(Rc::new(checker))),
    )));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
    )));

    let diffuse_light = DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        Box::new(diffuse_light),
    )));

    let world = BvhNode::from_list(world);

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 20.0;
    cam.look_from = Point3::new(26.0, 3.0, 6.0);
    cam.look_at = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::new();

    let left_red = Lambertian::new(Color::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new(Color::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new(Color::new(0.2, 0.8, 0.8));

    world.add(Rc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Box::new(left_red),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Box::new(back_green),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Box::new(right_blue),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Box::new(upper_orange),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Box::new(lower_teal),
    )));

    let world = BvhNode::from_list(world);

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 80.0;
    cam.look_from = Point3::new(0.0, 0.0, 9.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0));

    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Box::new(green),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Box::new(red),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        Box::new(light),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Box::new(white.clone()),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Box::new(white.clone()),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Box::new(white.clone()),
    )));

    let box1 = new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        Box::new(white.clone()),
    );
    let box1 = RotateY::new(Rc::new(box1), 15.0);
    let box1 = Translate::new(Rc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    world.add(Rc::new(box1));

    let box2 = new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        Box::new(white.clone()),
    );
    let box2 = RotateY::new(Rc::new(box2), -18.0);
    let box2 = Translate::new(Rc::new(box2), Vec3::new(130.0, 0.0, 65.0));

    world.add(Rc::new(box2));

    let world = BvhNode::from_list(world);

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Point3::new(278.0, 278.0, -800.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn perlin_spheres() {
    let mut world: HittableList = HittableList::new();

    let per_text = Rc::new(PerlinNoise::new(4.0));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::with_texture(per_text.clone())),
    )));

    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Box::new(Lambertian::with_texture(per_text)),
    )));

    let world = BvhNode::from_list(world);

    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    BouncingSpheres,
    CheckeredSpheres,
    SimpleLight,
    Quads,
    CornellBox,
    PerlinSpheres,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::BouncingSpheres => bouncing_spheres(),
        Command::CheckeredSpheres => checkered_spheres(),
        Command::SimpleLight => simple_light(),
        Command::Quads => quads(),
        Command::CornellBox => cornell_box(),
        Command::PerlinSpheres => perlin_spheres(),
    }
}
