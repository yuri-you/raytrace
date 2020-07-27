use crate::vec3::Vec3;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
#[derive(Clone, Debug, PartialEq)]
pub struct Sphere{
    pub center:Vec3,
    pub radius:f64,
}
impl Sphere{
    pub fn new(center:Vec3,radius:f64)->Self{
        Self{center,radius}
    }
}
impl Hittable for Sphere{
    fn hit(&mut self,r:&Ray, t_min:f64, t_max:f64, rec:&mut HitRecord)->bool{
    let oc = (*r).position.clone() - (*self).clone().center;
    let dir=(*r).direction.clone();
    let a = dir.squared_length();
    let half_b = oc.clone()*dir;
    let c = oc.clone().squared_length() - (*self).radius * (*self).radius ;
    let discriminant = half_b * half_b - a * c;
    if  discriminant > 0.0{
        let root = discriminant.sqrt();
        let mut temp = (-half_b - root) / a;
        if temp < t_max && temp > t_min {
            (*rec).t = temp;
            (*rec).p = (*r).clone().at(rec.t);
            let outward_normal:Vec3 = (rec.clone().p - (*self).clone().center) / (*self).clone().radius;
            rec.set_face_normal(r, &outward_normal);
            return true;
        }
        temp = (-half_b + root) / a;
        if temp < t_max && temp > t_min {
            (*rec).t = temp;
            (*rec).p = (*r).clone().at(rec.t);
            let outward_normal:Vec3 = ((*rec).clone().p - (*self).clone().center) / (*self).clone().radius;
            rec.set_face_normal(r, &outward_normal);
            return true;
        }
    }
    return false;
    }
}