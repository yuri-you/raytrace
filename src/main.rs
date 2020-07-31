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
mod texture;
mod camera;
mod perlin;
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
pub use texture::checker_texture;
pub use texture::noise_texture;
use std::sync::Arc;
pub fn random_scene()->HitList {

    let mut world=HitList::new();
    let checker1 = checker_texture::new2(Vec3::new(0.2, 0.3, 0.1),Vec3::new(0.9, 0.9, 0.9));
    let lam=lambertian::new1(Some(Arc::new(checker1)));
    let sph1=Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, Some(Arc::new(lam)));
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
                    let sph1=Moving_Sphere::new(center.clone(),center2,0.0,1.0, 0.2,Some(Arc::new(sphere_material)));
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
    let aspect_ratio=16.0/9.0;
    let image_width=400;
    let image_height=((image_width as f64)/aspect_ratio)as u32;
    let sample_per_pixel:i32=50;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);
    let maxdepth=10;
    //initialize

    let mut world:HitList;
    let mut lookfrom:Vec3;
    let mut lookat:Vec3;
    let mut vfov:f64;
    let mut aperture=0.0;
    let a=0;
    if a==1{
        world=random_scene();
        lookfrom = Vec3::new(13.0,2.0,3.0);
        lookat = Vec3::new(0.0,0.0,0.0);
        vfov = 20.0;
        aperture = 0.1;
    }
    else if a==2{
        world = two_spheres();
        lookfrom = Vec3::new(13.0,2.0,3.0);
        lookat = Vec3::new(0.0,0.0,0.0);
        vfov = 20.0;
    }
    else{
        world = two_perlin_spheres();
        lookfrom = Vec3::new(13.0,2.0,3.0);
        lookat = Vec3::new(0.0,0.0,0.0);
        vfov = 20.0;
    }
    let vup=Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let cam=Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus,0.0,1.0);
    //camera

    let mut u:f64;
    let mut v:f64;
    for x in 0..image_width {
        for y in 0..image_height {
            let pixel = img.get_pixel_mut(x, y);
            let mut color1=Vec3::new(0.0,0.0,0.0);
            let mut z=0;
            while z<sample_per_pixel{
            let mut q:f64=rand_double();
            u=((x as f64)+q)/((image_width-1)as f64);
            q=rand_double();
            v=(((image_height-1-y) as f64)+q)/((image_height-1)as f64);
            let rays=cam.get_ray(u, v);
            color1+=ray_color(&rays,&world,maxdepth);
            z+=1;
            }
            let color=write_color(color1,sample_per_pixel);
            *pixel = image::Rgb([color.x,color.y, color.z]);
        }
        bar.inc(1);
    }
    //picture


    img.save("output/test5.png").unwrap();
    bar.finish();
}

fn two_spheres()->HitList {
    let mut objects=HitList::new();

    let checker = Arc::new(checker_texture::new2(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));

    objects.add(Some(Arc::new(Sphere::new(Vec3::new(0.0,-10.0, 0.0), 10.0, Some(Arc::new(lambertian::new1(Some(checker.clone()))))))));
    objects.add(Some(Arc::new(Sphere::new(Vec3::new(0.0,10.0, 0.0), 10.0, Some(Arc::new(lambertian::new1(Some(checker))))))));

    return objects;
}
fn two_perlin_spheres()->HitList {
    let mut objects=HitList::new();
    let pertext =Arc::new(noise_texture::new1(4.0));
    objects.add(Some(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, Some(Arc::new(lambertian::new1(Some(pertext.clone()))))))));
    objects.add(Some(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Some(Arc::new(lambertian::new1(Some(pertext))))))));

    return objects;
}