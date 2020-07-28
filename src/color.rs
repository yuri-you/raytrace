use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::HitRecord;
use crate::hittable::HitList;
use crate::data::Infinity;
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
pub fn ray_color(r:&Ray,world:&HitList)->Vec3{
    let mut rec:HitRecord=HitRecord::new();
    if (*world).clone().hit(&r, 0.0, Infinity, &mut rec) {
        return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
    }
    let unit_direction:Vec3 = r.direction.clone().unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}
pub fn write_color(color1:Vec3,sample:i32)->Color{
    Color::new1(color1*256.0/(sample as f64))
}