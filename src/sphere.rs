use crate::vec3::Vec3;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::material::Material;
use crate::aabb::AABB;
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct Sphere{
    pub center:Vec3,
    pub radius:f64,
    pub ptr:Option<Arc<dyn Material>>,
}
impl Sphere{
    pub fn new(center:Vec3,radius:f64,ptr:Option<Arc<dyn Material>>)->Self{
        Self{center,radius,ptr,}
    }
    pub fn new1()->Self{
        Self{
        center: Vec3::new1(),
        radius:0.0,
        ptr:None,
        }
    }
    pub fn bounding_box(&self,t0:f64, t1:f64,output_box:&mut AABB) ->bool {
        let tmp=(*self).clone();
        (*output_box) = AABB::new(
            &(tmp.center.clone() - Vec3::new(tmp.radius.clone(), tmp.radius.clone(), tmp.radius.clone())),
            &(tmp.center.clone() + Vec3::new(tmp.radius.clone(), tmp.radius.clone(), tmp.radius.clone())));
        return true;
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
            (*rec).set_face_normal(r, &outward_normal);
            (*rec).ptr = (*self).ptr.clone();
            return true;
        }
        temp = (-half_b + root) / a;
        if temp < t_max && temp > t_min {
            (*rec).t = temp;
            (*rec).p = (*r).clone().at(rec.t);
            let outward_normal:Vec3 = ((*rec).clone().p - (*self).clone().center) / (*self).clone().radius;
            (*rec).set_face_normal(r, &outward_normal);
            (*rec).ptr = (*self).ptr.clone();
            return true;
        }
    }
    return false;
    }
}