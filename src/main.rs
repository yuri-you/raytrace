#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod color;
mod sphere;
mod hittable;
mod data;
mod camera;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use vec3::Vec3;
pub use ray::Ray;
pub use color::write_color;
pub use color::ray_color;
pub use color::Color;
pub use sphere::Sphere;
pub use hittable::HitList;
pub use camera::Camera;
pub use rand::prelude::*;
fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);
    // let viewheight:f64=2.0;
    // let viewwidth:f64=4.0;
    let mut world:HitList=HitList::new();
    world.add(&Sphere::new(Vec3::new(0.0,0.0, -1.0), 0.5));
    world.add(&Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0));
    let mut u:f64;
    let mut v:f64;
    // let mut random=Rand::new(14846);
    let camera=Camera::new();
    let sample_per_pixel:i32=100;
    let mut rng = rand::thread_rng();
    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            let mut color1=Vec3::new(0.0,0.0,0.0);
            let mut z=0;
            while z<sample_per_pixel{
            let mut q:f64=rng.gen();
            u=(((x as f64)+q)as i64)as f64;
            q=rng.gen();
            v=((((511-y) as f64)+q)as i64)as f64;
            let rays=camera.get_ray(u, v);
            color1+=ray_color(&rays,&mut world);
            z+=1;
            }
            let color=write_color(color1,sample_per_pixel);
            *pixel = image::Rgb([color.x,color.y, color.z]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
    //