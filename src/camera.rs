use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::data::degrees_to_radians;
use crate::vec3::random_in_unit_disk;
use crate::data::rand_range;
#[derive(Clone, Debug, PartialEq)]
pub struct Camera{
    origin:Vec3,
    lower_left_corner:Vec3,
    horizontal:Vec3,
    vertical:Vec3,
    u:Vec3,
    v:Vec3,
    w:Vec3,
    lens_radius:f64,
    t0:f64,
    t1:f64
}
impl Camera{
    pub fn new( lookfrom:Vec3,lookat:Vec3,vup:Vec3,vfov:f64,aspect_ratio:f64,aperture:f64, focus_dist:f64,t0:f64,t1:f64)->Self{
            let theta = degrees_to_radians(vfov);
            let h = (theta / 2.0).tan();
            let viewport_height:f64 = 2.0 * h;
            let viewport_width:f64 = aspect_ratio * viewport_height;
    
            let w1 = (lookfrom.clone() - lookat.clone()).unit();
            let u1 = vup.clone().cross(w1.clone()).unit();
            let v1 = w1.clone().cross(u1.clone());
            let or1=lookfrom;
            let ho1=focus_dist * viewport_width * u1.clone();
            let ver1=focus_dist * viewport_height * v1.clone();
            Self{
            origin: or1.clone(),    
            horizontal:ho1.clone(),
            vertical : ver1.clone(),
            lower_left_corner : or1.clone() - ho1.clone() / 2.0 - ver1.clone() / 2.0 - focus_dist * w1.clone(),
            u:u1,
            v:v1,
            w:w1,
            lens_radius : aperture / 2.0,
            t0,
            t1,
            }
    }
    pub fn get_ray(&self,s:f64,t:f64)->Ray{
        let rd = self.lens_radius.clone() * random_in_unit_disk();
        let offset = self.u.clone() * rd.x.clone() + self.v.clone() * rd.y.clone();
        Ray::new(self.origin.clone()+offset.clone(), self.lower_left_corner.clone() + s * self.horizontal.clone() + t * self.vertical.clone() - self.origin.clone()-offset.clone(),
        rand_range(self.t0,self.t1))
    }
}