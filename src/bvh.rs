use crate::hittable::Hittable;
use crate::aabb::AABB;
use std::sync::Arc;
use crate::hittable::HitList;
use crate::hittable::HitRecord;
// use crate::sphere::Sphere;
// use crate::moving_sphere::Moving_Sphere;
// use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::data::rand_int;
use crate::aabb::surrounding_box;
pub fn box_compare(a:Option<Arc<dyn Hittable>>, b:Option<Arc<dyn Hittable>>,axis:i32)->bool {
    let mut box_a=AABB::new1();
    let mut box_b=AABB::new1();
    if !(a.unwrap().bounding_box(0.0, 0.0, &mut box_a)) || !b.unwrap().bounding_box(0.0, 0.0,&mut box_b){
        println!( "No bounding box in bvh_node constructor.\n");
    }
    return box_a.min.in_at(axis) < box_b.min.in_at(axis);
}
pub fn sort(a:&mut Vec<Option<Arc<dyn Hittable>>>,start:i32,end:i32,axis:i32){
    if start>=end-1{return};
    let tmp=(*a)[start as usize].clone();
    let mut start1=start;
    let mut end1=end-1;
    while start1<end1{
        while start1<end1&& box_compare(tmp.clone(), (*a)[end1 as usize].clone(),axis){end1-=1;}
        if start1<end1{(*a)[start1 as usize]=(*a)[end1 as usize].clone();}
        while start1<end1&& box_compare((*a)[start1 as usize].clone(), tmp.clone(),axis){start1+=1;}
        if start1<end1{(*a)[end1 as usize]=(*a)[start1 as usize].clone();}
    }
    (*a)[start1 as usize]=tmp;
    sort(a,start,start1,axis);
    sort(a,start1+1,end1,axis);
}

pub fn box_x_compare(a:Option<Arc<dyn Hittable>>, b:Option<Arc<dyn Hittable>>) ->bool{
    box_compare(a, b, 0)
}

pub fn box_y_compare(a:Option<Arc<dyn Hittable>>, b:Option<Arc<dyn Hittable>>)->bool {
    box_compare(a, b, 1)
}

pub fn  box_z_compare(a:Option<Arc<dyn Hittable>>, b:Option<Arc<dyn Hittable>>)->bool {
    box_compare(a, b, 2)
}
#[derive(Clone)]
struct Bvh_node{
    pub left:Option<Arc<dyn Hittable>>,
    pub right:Option<Arc<dyn Hittable>>,
    pub box1:AABB,
}
impl Hittable for Bvh_node{
    fn bounding_box(&self,t0:f64, t1:f64, output_box:&mut AABB) ->bool {
        (*output_box) = (*self).box1.clone();
        return true;
    }

    fn hit(&self,r:&Ray, t_min:f64, t_max:f64, rec:&mut HitRecord)->bool{
    if !(*self).clone().box1.hit(r, t_min, t_max){return false;}

    let hit_left = (*self).clone().left.unwrap().hit(r, t_min, t_max, rec);
    let hit_right = (*self).clone().left.unwrap().hit(r, t_min, if hit_left {rec.t}else{t_max}, rec);

    return hit_left || hit_right;
    }
}
impl Bvh_node{
    pub fn new1()->Self{
        Self{
            left:None,
            right:None,
            box1:AABB::new1(),
        }
    }
    pub fn new2(&self,list:&HitList, time0:f64, time1:f64)->Self{
        Self::new(&mut (*list).ptr.clone(), 0, (*list).ptr.clone().len() as i32, time0, time1)
    }
    pub fn new(objects:&mut Vec<Option<Arc<dyn Hittable>>>,
        start:i32, end:i32, time0:f64, time1:f64)->Self{
            let mut self_tmp=Bvh_node::new1();
            let axis = rand_int(0, 2);
            // let comparator = if axis == 0 { box_x_compare}
            //     else if axis == 1 { box_y_compare}
            //     else {box_z_compare};
            // comparator(objects[0],objects[1]);
            let object_span = end - start;
            let mut b=(*objects).clone();
            if object_span == 1 {
                self_tmp.left =  b[start as usize].clone();
                self_tmp.right =  b[start as usize].clone();
            }
            else if object_span == 2 {
                if axis == 0 { 
                    if box_x_compare(b[start as usize].clone(), b[(start + 1)as usize].clone()){
                        self_tmp.left = b[start as usize].clone();
                        self_tmp.right =b[(start + 1)as usize].clone();
                    } 
                    else {
                        self_tmp.right = b[start as usize].clone();
                        self_tmp.left =b[(start + 1)as usize].clone();
                    }
                }
                else if axis == 1 { 
                    if box_y_compare(b[start as usize].clone(), b[(start + 1)as usize].clone()){
                        self_tmp.left = b[start as usize].clone();
                        self_tmp.right =b[(start + 1)as usize].clone();
                    } 
                    else {
                        self_tmp.right = b[start as usize].clone();
                        self_tmp.left =b[(start + 1)as usize].clone();
                    }
                }
                else {
                    if box_z_compare(b[start as usize].clone(), b[(start + 1)as usize].clone()){
                        self_tmp.left = b[start as usize].clone();
                        self_tmp.right =b[(start + 1)as usize].clone();
                    } 
                    else {
                        self_tmp.right = b[start as usize].clone();
                        self_tmp.left =b[(start + 1)as usize].clone();
                    }
                }
            }
            else {
                sort(&mut b,start, end,axis);
                let mid = start + object_span / 2;
                self_tmp.left =Some(Arc::new(Bvh_node::new(objects, start, mid, time0, time1)));
                self_tmp.right =Some(Arc::new(Bvh_node::new(objects, mid, end, time0, time1)));
            }
        
            let mut box_left:AABB=AABB::new1();
            let mut box_right:AABB=AABB::new1();
        
            if !self_tmp.left.clone().unwrap().bounding_box(time0, time1, &mut box_left)|| !self_tmp.right.clone().unwrap().bounding_box(time0, time1, &mut box_right)
                {
                println!( "No bounding box in bvh_node constructor.\n");
                }
            Self{
                left:self_tmp.left,
                right:self_tmp.right,
                box1:surrounding_box(&box_left, &box_right),
            }
        }
    }

