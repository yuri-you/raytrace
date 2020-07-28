use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::material::Material;
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct HitRecord{
    pub p:Vec3,
    pub normal:Vec3,
    pub t:f64,
    pub ptr:Option<Arc<dyn Material>>,
    pub front_face:bool,
}
impl HitRecord{
    pub fn new()->Self{
        Self{
            p:Vec3::new(0.0,0.0,0.0),
            normal:Vec3::new(0.0,0.0,0.0),
            t:0.0,
            ptr:None,
            front_face:false,
        }
    }
    pub fn set_face_normal(&mut self,r:&Ray,outward_normal:&Vec3){
        (*self).front_face = ((*r).direction.clone())*((*outward_normal).clone())< 0.0;
        (*self).normal = if self.front_face{(*outward_normal).clone()}else{-(*outward_normal).clone()};
    }
} 
pub trait Hittable{
    fn hit(&mut self,r:&Ray, t_min:f64, t_max:f64, rec:&mut HitRecord)->bool;
}
#[derive(Clone, Debug)]
pub struct HitList{
    pub ptr:Vec<Sphere>,
}
impl HitList{
    pub fn add(&mut self,sphere:&Sphere){
        (*self).ptr.push((*sphere).clone());
    }
    pub fn new()->Self{
        Self{
            ptr:Vec::new(),
        }
    }
    pub fn hit(mut self,r:&Ray,t_min:f64, t_max:f64, rec:&mut HitRecord)->bool{
        let mut temp_rec=HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for var in &mut self.ptr {
            if var.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything;
    }
}

