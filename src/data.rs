pub static Infinity:f64 = 0xfffffff as f64;
// pub static pi:f64= 3.1415926535897932385;
pub fn degrees_to_radians(degrees:f64)->f64{
    degrees * std::f64::consts::PI / 180.0
}
// #[derive(Clone, Debug, PartialEq)]
// pub struct Rand{
//     seed:i64,
//     a:i64,
//     b:i64,
//     c:i64,
//     md:i64
// }
// impl Rand{
//     pub fn new(seed:i64)->Self{
//         Self{
//             seed,
//             a:56,
//             b:37,
//             c:7465,
//             md:10000009
//         }
//     }
//     pub fn get(&mut self)->f64{
//         let tmp:i64=((*self).a*(*self).seed*(*self).seed+(*self).b*(*self).seed+(*self).c)%((*self).md);
//         (*self).seed=tmp;
//         (tmp as f64)/(((*self).md-1) as f64)
//     }
// }
pub use rand::prelude::*;
pub fn rand_double()->f64{
    let y:f64=rand::random::<f64>();
    y
}
pub fn rand_range(min:f64,max:f64)->f64{
    min+(max-min)*rand_double()
}