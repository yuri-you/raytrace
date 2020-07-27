use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]
pub struct Ray{
    pub position: Vec3,
    pub direction: Vec3,
}
impl Ray{
    pub fn new(pos:Vec3,dir:Vec3) -> Self {
        Self {position:pos.clone(),direction:dir.clone()}
    }
    pub fn at(self,t:f64)->Vec3{
        return self.position+self.direction*t;
    }
}