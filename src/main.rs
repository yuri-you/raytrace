#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;
pub use ray::Ray;
pub use color::write_color;
fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);
    let origin:Vec3=Vec3::new(0.0,0.0,0.0);
    // let viewheight:f64=2.0;
    // let viewwidth:f64=4.0;
    let low_left_corner=Vec3::new(-512.0 ,-256.0,-256.0);
    let horizontal=Vec3::new(1.0,0.0,0.0);
    let vertical=Vec3::new(0.0,1.0,0.0);
    let mut u:f64;
    let mut v:f64;
    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            u=x as f64;
            v=y as f64;
            let rays=Ray::new(origin.clone(),low_left_corner.clone()+horizontal.clone()*u+vertical.clone()*v);
            let color=write_color(rays);
            *pixel = image::Rgb([(color.x) as u8,(color.y) as u8, color.z as u8]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
    //