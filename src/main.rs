use ray_tracing::{
    new_box,
    utils::{self, random_double_range},
    BvhNode, Camera, Checkerboard, Color, Dielectric, DiffuseLight, HittableList, Lambertian,
    Material, Metal, PerlinNoise, Point3, Quadrilateral as Quad, RotateY, SolidColor, Sphere,
    Translate, Vec3,
};

use clap::{Parser, Subcommand};

use std::rc::Rc;

fn bouncing_spheres(image_width: usize) {
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
    cam.image_width = image_width;
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

fn checkered_spheres(image_width: usize) {
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
    cam.image_width = image_width;
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

fn simple_light(image_width: usize) {
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
    cam.image_width = image_width;
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

fn quads(image_width: usize) {
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
    cam.image_width = image_width;
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

fn cornell_box(image_width: usize) {
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
    cam.image_width = image_width;
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

fn perlin_spheres(image_width: usize) {
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
    cam.image_width = image_width;
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

fn final_scene() {
    let mut boxes1 = HittableList::new();
    let ground = Lambertian::new(Color::new(0.48, 0.83, 0.53));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f32) * w;
            let z0 = -1000.0 + (j as f32) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Rc::new(new_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Box::new(ground.clone()),
            )));
        }
    }
    let mut world = HittableList::new();
    world.add(Rc::new(BvhNode::from_list(boxes1)));

    let light = DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0));
    world.add(Rc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        Box::new(light),
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Rc::new(Sphere::moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Box::new(Dielectric::new(1.5)),
    )));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Box::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));
    /* TODO
    let boundary = Sphere::stationary(Point3::new(360.0,150.0,145.0), 70.0, Box::new(Dielectric::new(1.5)));
    world.add(Rc::new(boundary));
    world.add(Rc::new(constant_medium(boundary, 0.2, color(0.2, 0.4, 0.9))));
    boundary = make_shared<sphere>(point3(0,0,0), 5000, make_shared<dielectric>(1.5));
    world.add(make_shared<constant_medium>(boundary, .0001, color(1,1,1)));

    auto emat = make_shared<lambertian>(make_shared<image_texture>("earthmap.jpg"));
    world.add(make_shared<sphere>(point3(400,200,400), 100, emat));
    auto pertext = make_shared<noise_texture>(0.2);
    world.add(make_shared<sphere>(point3(220,280,300), 80, make_shared<lambertian>(pertext)));
    */

    let mut boxes2 = HittableList::new();
    let white = Box::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Rc::new(Sphere::stationary(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    world.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(Rc::new(BvhNode::from_list(boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let world: BvhNode = BvhNode::from_list(world);

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 250;
    cam.max_depth = 4;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Point3::new(478.0, 278.0, -600.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short = 'w', long, default_value_t = 400)]
    image_width: usize,
}

#[derive(Subcommand, Debug)]
enum Command {
    BouncingSpheres,
    CheckeredSpheres,
    SimpleLight,
    Quads,
    CornellBox,
    PerlinSpheres,
    FinalTest,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::BouncingSpheres => bouncing_spheres(args.image_width),
        Command::CheckeredSpheres => checkered_spheres(args.image_width),
        Command::SimpleLight => simple_light(args.image_width),
        Command::Quads => quads(args.image_width),
        Command::CornellBox => cornell_box(args.image_width),
        Command::PerlinSpheres => perlin_spheres(args.image_width),
        Command::FinalTest => final_scene(),
    }
}
