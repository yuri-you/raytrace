use crate::hittable::Hittable;
use crate::aabb::AABB;
use std::sync::Arc;
use crate::hittable::HitList;
use crate::vec3::Vec3;
use crate::data::rand_int;
pub fn box_compare(a:Option<Arc<dyn Hittable>>, b:Option<Arc<dyn Hittable>>,axis:i32)->bool {
    let mut box_a=AABB::new1();
    let mut box_b=AABB::new1();
    if !a.unwrap().bounding_box(0, 0, box_a) || !b.unwrap().bounding_box(0, 0, box_b){
        println!( "No bounding box in bvh_node constructor.\n");
    }
    return box_a.min.in_at(axis) < box_b.min.in_at(axis);
}


pub fn box_x_compare(a:Option<Arc<dyn Hittable>>, b:Option<Arc<dyn Hittable>>) ->bool{
    return box_compare(a, b, 0);
}

pub fn box_y_compare(a:Option<Arc<dyn Hittable>>, b:Option<Arc<dyn Hittable>>)->bool {
    return box_compare(a, b, 1);
}

pub fn  box_z_compare(a:Option<Arc<dyn Hittable>>, b:Option<Arc<dyn Hittable>>)->bool {
    return box_compare(a, b, 2);
}
#[derive(Clone)]
struct Bvh_node{
    pub left:Option<Arc<dyn Hittable>>,
    pub right:Option<Arc<dyn Hittable>>,
    pub box1:AABB,
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
        Self::new(self,&(*list).ptr.clone(), 0, (*list).ptr.clone().len() as i64, time0, time1)
    }
    pub fn new(&self,objects:&Vec<Option<Arc<dyn Hittable>>>,
        start:i64, end:i64, time0:f64, time1:f64)->Self{
            let axis = rand_int(0, 2);
            let comparator = if axis == 0 { box_x_compare}
                else if axis == 1 { box_y_compare}
                else {box_z_compare};
        
            let object_span = end - start;
            let b=(*objects).clone();
            if object_span == 1 {
                (*self).left = (*self).right = b[start];
            }
            else if object_span == 2 {
                if axis == 0 { 
                    if box_x_compare(b[start], b[start + 1]){
                        left = b[start];
                        right =b[start + 1];
                    } 
                    else {
                        left = b[start + 1];
                        right = b[start];
                    }
                }
                else if axis == 1 { 
                    if box_y_compare(b[start], b[start + 1]){
                        left = b[start];
                        right =b[start + 1];
                    } 
                    else {
                        left = b[start + 1];
                        right = b[start];
                    }
                }
                else {box_z_compare(b[start], b[start + 1]){
                    left = b[start];
                    right =b[start + 1];
                } 
                else {
                    left = b[start + 1];
                    right = b[start];
                }
                }
            }
            else {
                std::sort(objects.begin() + start, objects.begin() + end, comparator);
        
                auto mid = start + object_span / 2;
                left = make_shared<bvh_node>(objects, start, mid, time0, time1);
                right = make_shared<bvh_node>(objects, mid, end, time0, time1);
            }
        
            aabb box_left, box_right;
        
            if (!left->bounding_box(time0, time1, box_left)
                || !right->bounding_box(time0, time1, box_right)
                )
                std::cerr << "No bounding box in bvh_node constructor.\n";
        
            box = surrounding_box(box_left, box_right);
        }
    }


// bool bvh_node::bounding_box(double t0, double t1, aabb& output_box) const {
//     output_box = box;
//     return true;
// }
// bool bvh_node::hit(const ray& r, double t_min, double t_max, hit_record& rec) const {
//     if (!box.hit(r, t_min, t_max))
//         return false;

//     bool hit_left = left->hit(r, t_min, t_max, rec);
//     bool hit_right = right->hit(r, t_min, hit_left ? rec.t : t_max, rec);

//     return hit_left || hit_right;
// }
// bvh_node::bvh_node(
//     std::vector<shared_ptr<hittable>>& objects,
//     size_t start, size_t end, double time0, double time1
// ) {
    
// }

// #endif
