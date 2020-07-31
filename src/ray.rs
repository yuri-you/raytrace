use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]
pub struct Ray{
    pub position: Vec3,
    pub direction: Vec3,
    pub time:f64,
}
impl Ray{
    pub fn new(pos:Vec3,dir:Vec3,time:f64) -> Self {
        Self {position:pos.clone(),direction:dir.clone(),time}
    }
    pub fn new1()->Self{
        Self{
            position:Vec3::new1(),
            direction:Vec3::new1(),
            time:0.0,
        }
    }
    pub fn at(&self,t:f64)->Vec3{
        return self.position.clone()+self.direction.clone()*t;
    }
}