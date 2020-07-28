use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::HitRecord;
use crate::hittable::HitList;
use crate::data::Infinity;
use crate::vec3::random_in_unit_sphere;
use crate::vec3::random_in_hemisphere;
// use crate::data::pi;
#[derive(Clone, Debug, PartialEq)]
pub struct Color{
    pub x:u8,
    pub y:u8,
    pub z:u8,
}
impl Color{
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }
    pub fn new1(other:Vec3)->Self{
        Self{
            x:other.x as u8,
            y:other.y as u8,
            z:other.z as u8,
        }
    }
}
pub fn ray_color(r:&Ray,world:&HitList,depth:i32)->Vec3{
    let mut rec:HitRecord=HitRecord::new();
    if depth <= 0{
    return Vec3::new(0.0,0.0,0.0);
    }
    if (*world).clone().hit(&r, 0.00001, Infinity, &mut rec) {
        let mut scattered:Ray=Ray::new1();
        let mut attenuation:Vec3=Vec3::new1();
        if rec.clone().ptr.unwrap().scatter(&r, &rec,&mut attenuation,&mut scattered){
            return attenuation.elemul(ray_color(&scattered, &world, depth-1));
        }
        return Vec3::new(0.0,0.0,0.0);
    }
    let unit_direction:Vec3 = r.direction.clone().unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}
pub fn write_color(color1:Vec3,sample:i32)->Color{
    let scale = 1.0 / sample as f64;
    let r1 = (scale * color1.x).sqrt()*256.0;
    let g1 = (scale * color1.y).sqrt()*256.0;
    let b1 = (scale * color1.z).sqrt()*256.0;
    Color::new(r1 as u8,g1 as u8,b1 as u8)
}