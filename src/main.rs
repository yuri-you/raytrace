#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod color;
mod sphere;
mod hittable;
mod data;
mod aabb;
mod moving_sphere;
mod material;
mod bvh;
mod camera;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use vec3::Vec3;
pub use ray::Ray;
pub use color::write_color;
pub use color::ray_color;
pub use color::Color;
pub use sphere::Sphere;
pub use hittable::Hittable;
pub use hittable::HitList;
pub use camera::Camera;
pub use data::rand_double;
pub use data::rand_range;
pub use material::lambertian;
pub use material::metal;
pub use material::Material;
pub use material::dielectric;
pub use moving_sphere::Moving_Sphere;
use std::sync::Arc;
pub fn random_scene()->HitList {
    let mut world=HitList::new();
    let ground_material = lambertian::new(&Vec3::new(0.5, 0.5, 0.5));
    let sph1=Sphere::new(Vec3::new(0.0, -1000.0, 0.0),1000.0,Some(Arc::new(ground_material)));
    world.add(Some(Arc::new(sph1)));
    let mut a=-11;
    let mut b=-11;
    while a<11 {
        b=-11;
        while b < 11 {
            let choose_mat = rand_double();
            let center=Vec3::new(a as f64 + 0.9*rand_double(), 0.2, b as f64 + 0.9*rand_double());
            if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9{
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::rand_double().elemul(Vec3::rand_double());
                    let sphere_material=lambertian::new(&albedo);   
                    let center2 = center.clone() + Vec3::new(0.0, rand_range(0.0,0.5), 0.0);
                    // world.add(make_shared<moving_sphere>(
                    //     center, center2, 0.0, 1.0, 0.2, sphere_material));
                    let sph1=Moving_Sphere::new(center.clone(),center2.clone(),0.0,1.0, 0.2,Some(Arc::new(sphere_material)));
                    world.add(Some(Arc::new(sph1)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::rand_range(0.5, 1.0);
                    let fuzz = rand_range(0.0, 0.5);
                    let sphere_material = metal::new(&albedo, fuzz);
                    let sph1=Sphere::new(center,0.2,Some(Arc::new(sphere_material)));
                    world.add(Some(Arc::new(sph1)));
                } else {
                    // glass
                    let sphere_material = dielectric::new(1.5);
                    let sph1=Sphere::new(center,0.2,Some(Arc::new(sphere_material)));
                    world.add(Some(Arc::new(sph1)));
                }
            }
            b+=1;
        }
        a+=1;
    }

    let material1 = dielectric::new(1.5);
    let sph1=Sphere::new(Vec3::new(0.0,1.0,0.0),1.0,Some(Arc::new(material1)));
    world.add(Some(Arc::new(sph1)));

    let material2 = lambertian::new(&Vec3::new(0.4, 0.2, 0.1));
    let sph2=Sphere::new(Vec3::new(-4.0,1.0,0.0),1.0,Some(Arc::new(material2)));
    world.add(Some(Arc::new(sph2)));

    let material3 = metal::new(&Vec3::new(0.7, 0.6, 0.5),0.0);
    let sph3=Sphere::new(Vec3::new(4.0,1.0,0.0),1.0,Some(Arc::new(material3)));
    world.add(Some(Arc::new(sph3)));

    // let material4=metal::new(&Vec3::new(0.6, 0.5, 0.6),0.0);
    // world.add(&Sphere::new(Vec3::new(0.0,-1.0,0.0), 0.7,Some(Arc::new(material4))));
    return world;
}
fn main() {
    
    // let x = Vec3::new(1.0, 1.0, 1.0);
    // println!("{:?}", x);
    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let aspect_ratio = 2.0;
    let image_width = 1024;
    let image_height=512;
    let bar = ProgressBar::new(1024);
    let sample_per_pixel:i32=100;
    let max_depth=50;
    //initialize


    let mut world:HitList=random_scene();  
    // let material_ground=lambertian::new(&Vec3::new(0.8, 0.8, 0.0));
    // let material_center=lambertian::new(&Vec3::new(0.1, 0.2, 0.5));
    // let material_left= dielectric::new(1.5);
    // let material_right = metal::new(&Vec3::new(0.8, 0.6, 0.2),0.0);
    // world.add(&Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0,Some(Arc::new(material_ground))));
    // world.add(&Sphere::new(Vec3::new( 0.0,   0.0, -1.0), 0.5,Some(Arc::new(material_center))));
    // world.add(&Sphere::new(Vec3::new(-1.0,    0.0, -1.0), 0.5,Some(Arc::new(material_left.clone()))));
    // world.add(&Sphere::new(Vec3::new(-1.0,    0.0, -1.0), -0.4,Some(Arc::new(material_left))));
    // world.add(&Sphere::new(Vec3::new(1.0,    0.0, -1.0), 0.5,Some(Arc::new(material_right))));
    //world
    let lookfrom=Vec3::new(13.0,2.0,3.0);
    let lookat=Vec3::new(0.0,0.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    
    let cam=Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus,0.0,1.0);
    let mut u:f64;
    let mut v:f64;
    // let mut random=Rand::new(14846);
    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            let mut color1=Vec3::new(0.0,0.0,0.0);
            let mut z=0;
            while z<sample_per_pixel{
            u=(((x as f64)+rand_double())as i64)as f64;
            v=((((511-y) as f64)+rand_double())as i64)as f64;
            let rays=cam.get_ray(u, v);
            color1+=ray_color(&rays,&mut world,max_depth);
            z+=1;
            }
            let color=write_color(color1,sample_per_pixel);
            *pixel = image::Rgb([color.x,color.y, color.z]);
        }
        bar.inc(1);
    }

    img.save("output/test2.png").unwrap();
    bar.finish();
}
    //