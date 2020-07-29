use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::material::Material;
use crate::aabb::surrounding_box;
use crate::aabb::AABB;
use std::sync::Arc;
#[derive(Clone)]
pub struct HitRecord{
    pub p:Vec3,
    pub normal:Vec3,
    pub t:f64,
    pub ptr:Option<Arc<dyn Material>>,
    pub front_face:bool,
    pub u:f64,
    pub v:f64,
}
impl HitRecord{
    pub fn new()->Self{
        Self{
            p:Vec3::new(0.0,0.0,0.0),
            normal:Vec3::new(0.0,0.0,0.0),
            t:0.0,
            ptr:None,
            front_face:false,
            u:0.0,
            v:0.0,
        }
    }
    pub fn set_face_normal(&mut self,r:&Ray,outward_normal:&Vec3){
        (*self).front_face = ((*r).direction.clone())*((*outward_normal).clone())< 0.0;
        (*self).normal = if self.front_face{(*outward_normal).clone()}else{-(*outward_normal).clone()};
    }
} 
pub trait Hittable{
    fn hit(&self,r:&Ray, t_min:f64, t_max:f64, rec:&mut HitRecord)->bool;
    fn bounding_box(&self,t0:f64, t1:f64,output_box:&mut AABB)->bool;
}
#[derive(Clone)]
pub struct HitList{
    pub ptr:Vec<Option<Arc<dyn Hittable>>>,
}
impl HitList{
    pub fn add(&mut self,m:Option<Arc<dyn Hittable>>){
        (*self).ptr.push(m);
    }
    pub fn new()->Self{
        Self{
            ptr:Vec::new(),
        }
    }
    pub fn hit(&mut self,r:&Ray,t_min:f64, t_max:f64, rec:&mut HitRecord)->bool{
        let mut temp_rec=HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for var in (*self).clone().ptr {
            if var.clone().unwrap().hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything;
    }
    pub fn bounding_box(&self,t0:f64, t1:f64, output_box:&mut AABB) ->bool {
        if (*self).ptr.len()==0 {return false;}
        let mut temp_box=AABB::new1();
        let mut first_box = true;
        for  object in (*self).ptr.iter() {
            if  object.clone().unwrap().bounding_box(t0, t1, &mut temp_box) {return false;}
            let output=(*output_box).clone();
            (*output_box) = if first_box{ temp_box.clone()}else{surrounding_box(&output, &temp_box)};
            first_box = false;
        }
    
        return true;
    }
}

