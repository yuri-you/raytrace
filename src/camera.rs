use crate::vec3::Vec3;
use crate::ray::Ray;
#[derive(Clone, Debug, PartialEq)]
pub struct Camera{
    origin:Vec3,
    lower_left_corner:Vec3,
    horizontal:Vec3,
    vertical:Vec3,
}
impl Camera{
    pub fn new()->Self{
        Self{
            origin:Vec3::new(0.0,0.0,0.0),
            lower_left_corner:Vec3::new(-512.0 ,-256.0,-256.0),
            horizontal:Vec3::new(1.0,0.0,0.0),
            vertical:Vec3::new(0.0,1.0,0.0),
        }
    }
    pub fn get_ray(&self,u:f64,v:f64)->Ray{
        Ray::new((*self).clone().origin, (*self).clone().lower_left_corner + u * (*self).clone().horizontal + v * (*self).clone().vertical - (*self).clone().origin)
    }
}