#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod color;
mod sphere;
mod hittable;
mod data;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use vec3::Vec3;
pub use ray::Ray;
pub use color::write_color;
pub use color::ray_color;
pub use color::Color;
pub use sphere::Sphere;
pub use hittable::HitList;
fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);
    let origin:Vec3=Vec3::new(0.0,0.0,0.0);
    // let viewheight:f64=2.0;
    // let viewwidth:f64=4.0;
    let mut world:HitList=HitList::new();
    world.add(&Sphere::new(Vec3::new(0.0,0.0, -1.0), 0.5));
    world.add(&Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0));
    let low_left_corner=Vec3::new(-512.0 ,-256.0,-256.0);
    let horizontal=Vec3::new(1.0,0.0,0.0);
    let vertical=Vec3::new(0.0,1.0,0.0);
    let mut u:f64;
    let mut v:f64;
    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            u=x as f64;
            v=(511-y) as f64;
            let rays=Ray::new(origin.clone(),low_left_corner.clone()+horizontal.clone()*u+vertical.clone()*v);
            let color1=ray_color(&rays,&mut world);
            let color=write_color(color1);
            *pixel = image::Rgb([color.x,color.y, color.z]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
    //