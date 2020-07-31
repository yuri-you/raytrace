use crate::vec3::Vec3;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::material::Material;
use std::sync::Arc;
use crate::aabb::surrounding_box;
use crate::aabb::AABB;
#[derive(Clone)]
pub struct Moving_Sphere{
    pub center0:Vec3,
    pub center1:Vec3,
    pub radius:f64,
    pub ptr:Option<Arc<dyn Material>>,
    pub t0:f64,
    pub t1:f64,
}
impl Moving_Sphere{
    pub fn new(center0:Vec3,center1:Vec3,t0:f64,t1:f64,radius:f64,ptr:Option<Arc<dyn Material>>)->Self{
        Self{center0,center1,radius,ptr,t0,t1}
    }
    pub fn new1()->Self{
        Self{
        center0: Vec3::new1(),
        center1:Vec3::new1(),
        radius:0.0,
        ptr:None,
        t0:0.0,
        t1:0.0,
        }
    }
    pub fn center(&self,t0:f64)->Vec3{
        self.center0.clone() + (((t0 - self.t0) / (self.t1 - self.t0)) * (self.center1.clone() - self.center0.clone()))
        }
}
impl Hittable for Moving_Sphere{
    fn hit(&self,r:&Ray, t_min:f64, t_max:f64, rec:&mut HitRecord)->bool{
    let oc = r.position.clone() - self.center(r.time);
    let dir=r.direction.clone();
    let a = dir.squared_length();
    let half_b = oc.clone()*dir;
    let c = oc.clone().squared_length() - self.radius * self.radius ;
    let discriminant = half_b * half_b - a * c;
    if  discriminant > 0.0{
        let root = discriminant.sqrt();
        let mut temp = (-half_b - root) / a;
        if temp < t_max && temp > t_min {
            rec.t = temp;
            rec.p = r.at(rec.t);
            let outward_normal:Vec3 = (rec.p.clone() - self.center(r.time)) / self.radius.clone();
            rec.set_face_normal(r, &outward_normal);
            rec.ptr = self.ptr.clone();
            return true;
        }
        temp = (-half_b + root) / a;
        if temp < t_max && temp > t_min {
            rec.t = temp;
            rec.p = r.at(rec.t);
            let outward_normal:Vec3 = (rec.p.clone() - self.center(r.time)) / self.radius.clone();
            rec.set_face_normal(r, &outward_normal);
            rec.ptr = self.ptr.clone();
            return true;
        }
    }
    return false;
    }
    fn bounding_box(&self,t0:f64, t1:f64,output_box:&mut AABB) ->bool {
        let box0 = AABB::new(
            &(self.center(t0) - Vec3::new(self.radius.clone(), self.radius.clone(), self.radius.clone())),
            &(self.center(t0) + Vec3::new(self.radius.clone(), self.radius.clone(), self.radius.clone())));
        let box1 = AABB::new(
            &(self.center(t1) - Vec3::new(self.radius.clone(), self.radius.clone(), self.radius.clone())),
            &(self.center(t1) + Vec3::new(self.radius.clone(), self.radius.clone(), self.radius.clone())));
        (*output_box)=surrounding_box(&box0, &box1);
        return true;
    }
}