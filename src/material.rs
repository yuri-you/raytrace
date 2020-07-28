use crate::hittable::HitRecord;
use crate::vec3::random_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::random_in_unit_sphere;
use crate::vec3::reflect;
use crate::vec3::refract;
use crate::data::rand_double;
pub trait Material : std::fmt::Debug{
    fn scatter(&self,r_in:&Ray,rec:&HitRecord , attenuation:&mut Vec3 ,scattered:&mut Ray)->bool;
}
pub fn fmin(a:f64,b:f64)->f64{
    if a>b{b}else{a}
}
pub fn schlick(cosine:f64, ref_idx:f64)->f64 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    return r0 + (1.0-r0)*((1.0 - cosine).powf(5.0));
}
#[derive(Clone, Debug, PartialEq)]
pub struct lambertian{
    albedo:Vec3,
}
impl Material for lambertian{
    fn scatter(&self,r_in:&Ray,rec:&HitRecord , attenuation:&mut Vec3 ,scattered:&mut Ray)->bool{
        let scatter_direction = (*rec).normal.clone() + random_unit_sphere();
        (*scattered) = Ray::new((*rec).p.clone(), scatter_direction);
        (*attenuation) = (*self).albedo.clone();
        return true;
    }
}
impl lambertian{
    pub fn new(al:&Vec3)->Self{
        Self{albedo:(*al).clone(),}
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct metal{
    albedo:Vec3,
    fuzz:f64,
}
impl Material for metal{
    fn scatter(&self,r_in:&Ray,rec:&HitRecord , attenuation:&mut Vec3 ,scattered:&mut Ray)->bool{
        let reflected = reflect(&(r_in.direction.clone().unit()), &(*rec).normal.clone());
        (*scattered) = Ray::new((*rec).p.clone(), reflected + self.fuzz * random_in_unit_sphere());
        (*attenuation) = self.albedo.clone();
        return scattered.clone().direction*((*rec).normal.clone()) > 0.0;
    }
}
impl metal{
    pub fn new(al:&Vec3,fuzz1:f64)->Self{
        Self{
            albedo:(*al).clone(),
            fuzz:if fuzz1<1.0{fuzz1}else{1.0},
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct dielectric{
    ref_idx:f64,
} 
impl Material for dielectric{
    fn scatter(&self,r_in:&Ray,rec:&HitRecord , attenuation:&mut Vec3 ,scattered:&mut Ray)->bool{
        (*attenuation) = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat:f64 = if(*rec).front_face.clone(){1.0 / (*self).ref_idx}else{ (*self).ref_idx};
        let unit_direction = (*r_in).clone().direction.unit();
        let cos_theta = fmin((-(unit_direction.clone()))*(*rec).clone().normal, 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &rec.normal);
            (*scattered) = Ray::new((*rec).clone().p, reflected);
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if rand_double() < reflect_prob
        {
            let reflected = reflect(&unit_direction, &(*rec).clone().normal);
            (*scattered) = Ray::new((*rec).clone().p, reflected);
            return true;
        }
        let refracted = refract(&unit_direction, &rec.normal, etai_over_etat);
        (*scattered) = Ray::new((*rec).clone().p, refracted);
        return true;
    }
}
impl dielectric{
    pub fn new(ref_idx:f64)->Self{
        Self{
            ref_idx,
        }
    }
}