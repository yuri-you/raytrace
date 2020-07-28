use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::data::min;
use crate::data::max;
pub fn swap(a:&mut f64,b:&mut f64){
    let c=*a;
    (*a)=*b;
    (*b)=c;
}
#[derive(Clone, Debug, PartialEq)]
pub struct AABB {
     pub min:Vec3,
     pub max:Vec3,
 }
 impl AABB{
     pub fn new(a:&Vec3,b:&Vec3)->Self{
        Self{
            min:(*a).clone(),
            max:(*b).clone(),
        }
     }
     pub fn new1()->Self{
         Self{
             min:Vec3::new1(),
             max:Vec3::new1(),
         }
     }
     pub fn hit(&self,r:&Ray,tin:f64,tax:f64)->bool{
        let mut a=0;
        let mut tmin=tin;
        let mut tmax=tax;
        let tmp=(*self).clone();
        while a<3 {
            let invD = 1.0/ (*r).clone().direction.in_at(a);
        let mut t0 = (tmp.min.in_at(a) - (*r).clone().position.in_at(a)) * invD;
        let mut t1 = (tmp.max.in_at(a) - (*r).clone().position.in_at(a)) * invD;
        if invD < 0.0{
            swap(&mut t0, &mut t1);
        }
        tmin = if t0 > tmin {t0}else{ tmin};
        tmax = if t1 < tmax {t1}else {tmax};
        if tmax <= tmin{
            return false;
        }
        a+=1;
        }
        return true;
     }
 }
pub fn surrounding_box(box0:&AABB, box1:&AABB)->AABB {
    let small=Vec3::new(min((*box0).clone().min.x, (*box1).clone().min.x),
    min((*box0).clone().min.y, (*box1).clone().min.y),
    min((*box0).clone().min.z, (*box1).clone().min.z));

    let big=Vec3::new(max((*box0).clone().max.x, (*box1).clone().max.x),
    max((*box0).clone().max.y, (*box1).clone().max.y),
    max((*box0).clone().max.z, (*box1).clone().max.z));

    return AABB::new(&small, &big);
}