pub static infinity:f64 = 0xffffff as f64;
pub static pi:f64= 3.1415926535897932385;
pub fn degrees_to_radians(degrees:f64)->f64{
    degrees * pi.clone() / 180.0
}