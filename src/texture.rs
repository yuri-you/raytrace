use crate::vec3::Vec3;
use std::sync::Arc;
use crate::perlin::Perlin;
pub trait texture {
    fn value(&self,u:f64, v:f64,p:&Vec3)->Vec3;
}

#[derive(Clone, Debug, PartialEq)]
pub struct noise_texture{
    noise:Perlin,
    scale:f64,
}
impl noise_texture{
    pub fn new()->Self{
        Self{
            noise:Perlin::new(),
            scale:0.0,
        }
    }

    pub fn new1(scale:f64)->Self{
        Self{
            noise:Perlin::new(),
            scale,
        }
    }
}
impl texture for noise_texture{
    fn value(&self,u: f64, v: f64, p: &Vec3)->Vec3{
        Vec3::new(1.0,1.0,1.0)*(self.noise.noise(&(self.scale*(*p).clone())))
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct solid_color{
    pub color_value:Vec3,
}
impl solid_color{
    pub fn new1()->Self{
        Self{
            color_value:Vec3::new1(),
        }
    }
    pub fn new(c:Vec3)->Self{
        Self{
            color_value:c,
        }
    }
    pub fn new2(red:f64,green:f64,blue:f64)->Self{
        Self{
            color_value:Vec3::new(red,green,blue),
        }
    }
}
impl texture for solid_color{
    fn value(&self,u:f64, v:f64,p:&Vec3)->Vec3{
        (*self).color_value.clone()
    }
}
#[derive(Clone)]
pub struct checker_texture{
    pub  odd:Option<Arc<dyn texture>>,
    pub  even:Option<Arc<dyn texture>>,
}
impl checker_texture{
    pub fn new1()->Self{
        Self{
            odd:None,
            even:None,
        }
    }
    pub fn new(odd1:&Option<Arc<dyn texture>>,even1:&Option<Arc<dyn texture>>)->Self{
        Self{
            odd:(*odd1).clone(),
            even:(*even1).clone(),
        }
    }
    pub fn new2(odd1:Vec3,even1:Vec3)->Self{
        Self{
            odd:Some(Arc::new(solid_color::new(odd1))),
            even:Some(Arc::new(solid_color::new(even1))),
        }
    }
}
impl texture for checker_texture{
    fn value(&self,u:f64, v:f64,p:&Vec3)->Vec3{
         let sines = (10.0 * (*p).clone().x).sin() * (10.0 * (*p).clone().y).sin() * (10.0 * (*p).clone().z).sin();
            if  sines < 0.0{
                return (*self).clone().odd.unwrap().value(u, v, p);
            }
            else{
                return (*self).clone().even.unwrap().value(u, v, p);
            }
        }
}
