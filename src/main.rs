use ray_tracing::{
    new_box, utils, vec3, BvhNode, Camera, Checkerboard, Color, Config, ConstantMedium, Dielectric,
    DiffuseLight, HittableList, Lambertian, Metal, NoneMaterial, PerlinNoise, Point3,
    Quadrilateral as Quad, RotateY, SolidColor, Sphere, Translate, Vec3,
};

use clap::{Parser, Subcommand};

use std::sync::Arc;

fn bouncing_spheres(image_width: usize, file_path: String) {
    let mut world: HittableList = HittableList::new();

    let even = Arc::new(SolidColor::from_rgb(0.2, 0.3, 0.1));
    let odd = Arc::new(SolidColor::from_rgb(0.9, 0.9, 0.9));
    let checker = Checkerboard::new(even, odd, 0.32);
    let material_ground = Lambertian::with_texture(Arc::new(checker));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(material_ground),
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
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 =
                        center + Point3::new(0.0, utils::random_double_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = utils::random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::stationary(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::stationary(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.0 / 1.5);
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        0.5,
        Arc::new(material1),
    )));

    let material2 = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(material2),
    )));

    let material3 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(material3),
    )));

    let material4 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(material4),
    )));

    let world = BvhNode::from_list(world);
    let lights = Arc::new(HittableList::new());

    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.file_path = file_path;
    cam.render(&world, lights);
}

fn checkered_spheres(image_width: usize, file_path: String) {
    let mut world: HittableList = HittableList::new();
    let checker = Checkerboard::new(
        Arc::new(SolidColor::from_rgb(0.2, 0.3, 0.1)),
        Arc::new(SolidColor::from_rgb(0.9, 0.9, 0.9)),
        0.32,
    );
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::with_texture(Arc::new(checker.clone()))),
    )));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::with_texture(Arc::new(checker))),
    )));

    let _world = BvhNode::from_list(world);

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

    cam.file_path = file_path;
    // cam.render(&world);
}

fn simple_light(image_width: usize, file_path: String) {
    let mut world: HittableList = HittableList::new();
    let checker = Checkerboard::new(
        Arc::new(SolidColor::from_rgb(0.2, 0.3, 0.1)),
        Arc::new(SolidColor::from_rgb(0.9, 0.9, 0.9)),
        0.32,
    );
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::with_texture(Arc::new(checker))),
    )));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
    )));

    let diffuse_light = DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        Arc::new(diffuse_light),
    )));

    let _world = BvhNode::from_list(world);

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

    cam.file_path = file_path;
    // cam.render(&world);
}

fn quads(image_width: usize, file_path: String) {
    let mut world = HittableList::new();

    let left_red = Lambertian::new(Color::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new(Color::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new(Color::new(0.2, 0.8, 0.8));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(left_red),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(back_green),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(right_blue),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Arc::new(upper_orange),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Arc::new(lower_teal),
    )));

    let _world = BvhNode::from_list(world);

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

    cam.file_path = file_path;
    // cam.render(&world);
}

fn perlin_spheres(image_width: usize, file_path: String) {
    let mut world: HittableList = HittableList::new();

    let per_text = Arc::new(PerlinNoise::new(4.0));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::with_texture(per_text.clone())),
    )));

    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::with_texture(per_text)),
    )));

    let _world = BvhNode::from_list(world);

    let mut cam: Camera = Camera::new();
    // cam.aspect_ratio = 16. / 9.;
    cam.aspect_ratio = 1.;
    cam.image_width = image_width;
    cam.samples_per_pixel = 1000;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.file_path = file_path;
    // cam.render(&world);
}

fn cornell_smoke(image_width: usize, file_path: String) {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));

    // Cornell box walls
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Create boxes with transformations
    let box1 = new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(Arc::new(box1), 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let box2 = new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Arc::new(RotateY::new(Arc::new(box2), -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    // Add volumetric media (smoke)
    world.add(Arc::new(ConstantMedium::with_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    world.add(Arc::new(ConstantMedium::with_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    let _world: BvhNode = BvhNode::from_list(world);

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

    cam.file_path = file_path;
    // cam.render(&world);
}

fn cornell_box(file_path: String) {
    let toml_string = std::fs::read_to_string(file_path).expect("couldn't open file");
    let config: Config = toml::from_str(&toml_string).expect("invalid config file");
    let (mut camera, mut world) = config.to_scene().expect("invalid scene");

    let mut lights = HittableList::new();

    let empty_mat = Arc::new(NoneMaterial);
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));

    /*
    let metal = Metal::new(Color::new(0.8, 0.85, 0.88), 0.0);
    let box1 = new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        Arc::new(metal),
    );
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    world.add(Arc::new(box1));
    */

    // let box2 = new_box(
    //     Point3::new(0.0, 0.0, 0.0),
    //     Point3::new(165.0, 165.0, 165.0),
    //     Arc::new(white.clone()),
    // );
    // let box2 = RotateY::new(Arc::new(box2), -18.0);
    // let box2 = Translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));
    // world.add(Arc::new(box2));

    /*
    let glass = Arc::new(Dielectric::new(0.));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(190., 90., 190.),
        90.,
        glass,
    )));
    */

    world.add(Arc::new(Quad::new(
        Point3::new(343., 554., 332.),
        Vec3::new(-130., 0., 0.),
        Vec3::new(0., 0., -105.),
        light,
    )));
    lights.add(Arc::new(Quad::new(
        Point3::new(343., 554., 332.),
        Vec3::new(-130., 0., 0.),
        Vec3::new(0., 0., -105.),
        empty_mat.clone(),
    )));

    let world = BvhNode::from_list(world);

    camera.render(&world, Arc::new(lights));
}

fn final_scene(image_width: usize, file_path: String) {
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
            let y1 = utils::random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(new_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::new(ground.clone()),
            )));
        }
    }
    let mut world = HittableList::new();
    world.add(Arc::new(BvhNode::from_list(boxes1)));

    let light = DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0));
    let empty_mat = Arc::new(NoneMaterial);

    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        Arc::new(light.clone()),
    )));
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        empty_mat,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::stationary(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::with_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::stationary(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::with_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    /* TODO
    auto emat = make_shared<lambertian>(make_shared<image_texture>("earthmap.jpg"));
    world.add(make_shared<sphere>(point3(400,200,400), 100, emat));
    auto pertext = make_shared<noise_texture>(0.2);
    world.add(make_shared<sphere>(point3(220,280,300), 80, make_shared<lambertian>(pertext)));
    */

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::stationary(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::from_list(boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let world: BvhNode = BvhNode::from_list(world);

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = 1_000;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Point3::new(478.0, 278.0, -600.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.file_path = file_path;
    cam.render(&world, Arc::new(lights));
}

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short = 'w', long, default_value_t = 400)]
    image_width: usize,

    #[arg(short = 'f', long)]
    file_path: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    BouncingSpheres,
    CheckeredSpheres,
    SimpleLight,
    Quads,
    CornellBox,
    CornellSmoke,
    PerlinSpheres,
    FinalTest,
    PiTest,
}

fn pi_test() {
    fn f(d: Vec3) -> f32 {
        d.z() * d.z()
    }
    fn pdf(_: Vec3) -> f32 {
        1. / (4. * std::f32::consts::PI)
    }

    let n = 100_000;
    let mut sum = 0.;

    for _ in 0..n {
        let d = vec3::random_unit_vector();
        let f_d = f(d);
        sum += f_d / pdf(d);
    }

    println!("I = {}", sum / (n as f32));
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::BouncingSpheres => bouncing_spheres(args.image_width, args.file_path),
        Command::CheckeredSpheres => checkered_spheres(args.image_width, args.file_path),
        Command::SimpleLight => simple_light(args.image_width, args.file_path),
        Command::Quads => quads(args.image_width, args.file_path),
        Command::CornellBox => cornell_box(args.file_path),
        Command::CornellSmoke => cornell_smoke(args.image_width, args.file_path),
        Command::PerlinSpheres => perlin_spheres(args.image_width, args.file_path),
        Command::FinalTest => final_scene(args.image_width, args.file_path),
        Command::PiTest => pi_test(),
    }
}
