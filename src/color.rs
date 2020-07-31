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
    if world.hit(&r, 0.00001, Infinity, &mut rec) {
        let mut scattered:Ray=Ray::new1();
        let mut attenuation:Vec3=Vec3::new1();
        if rec.ptr.clone().unwrap().scatter(&r, &rec,&mut attenuation,&mut scattered){
            return attenuation.elemul(ray_color(&scattered, &world, depth-1));
        }
        return Vec3::new(0.0,0.0,0.0);
    }
    let unit_direction:Vec3 = r.direction.clone().unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

pub fn clamp(x:f64,min:f64, max:f64)->f64{
    if x < min{ return min;}
    else if x > max { return max;}
    else {return x;}
}

pub fn write_color(color1:Vec3,sample:i32)->Color{
    let r=color1.x;
    let g=color1.y;
    let b=color1.z;
    let p=1.0/(sample as f64);
    let r1=(p*r).sqrt();
    let g1=(p*g).sqrt();
    let b1=(p*b).sqrt();
    return Color::new(
        (256.0 * clamp(r1, 0.0, 0.999)) as u8,
        (256.0 * clamp(g1, 0.0, 0.999)) as u8,
        (256.0 * clamp(b1, 0.0, 0.999)) as u8,
    )
}