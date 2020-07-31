use crate::data::rand_double;
use crate::data::rand_int;
use crate::vec3::Vec3;
static point_count:i32=256;
fn floor(p:f64)->f64{
    if p>=0.0{
        (p as i64)as f64
    }
    else{
        ((p as i64)-1) as f64
    }
}
fn permute(p:&mut Vec<i32>, n:i32) {
   for  i in 0..n{
        let j=n-1-i;
        let target = rand_int(0, i);
            let tmp = (*p)[j as usize];
            (*p)[j as usize] = (*p)[target as usize];
            (*p)[target as usize] = tmp;
    }
}
fn trilinear_interp(c:&Vec<f64>, u:f64, v:f64, w:f64)->f64 {
    let mut accum = 0.0;
    for  i in 0..2{
        for  j in 0..2{
            for  k in 0..2{
                accum += ((i as f64)*u + (1.0-i as f64)*(1.0-u))* (j as f64*v + (1.0-j as f64)*(1.0-v))*(k as f64*w + (1.0-k as f64)*(1.0-w))
                *c[(i*4+j*2+k) as usize];
            }
        }
    }
    return accum;
}
#[derive(Clone, Debug, PartialEq)]
pub struct Perlin {
    ranfloat:Vec<f64>,
    perm_x:Vec<i32>,
    perm_y:Vec<i32>,
    perm_z:Vec<i32>,
}
impl Perlin{
    pub fn new1()->Self{
        Self{
            ranfloat:vec![],
            perm_x:vec![],
            perm_y:vec![],
            perm_z:vec![],
        }
    }
    pub fn new()->Self{
        let mut tmp:Vec<f64>=vec![0.0;point_count as usize];
        for i in 0..256{
            tmp[i]=rand_double();
        }
        let a:Perlin=Perlin::new1();
        Self{
            ranfloat:tmp,
            perm_x:a.perlin_generate_perm(),
            perm_y:a.perlin_generate_perm(),
            perm_z:a.perlin_generate_perm(),
        }
    }
    pub fn perlin_generate_perm(&self)->Vec<i32>{
        let mut p = vec![0;256];

        for  i in 0..point_count{
            p[i as usize] = i;
        }
        permute(&mut p, point_count);

        return p;
    }
    pub fn noise(&self,p:&Vec3)->f64{
        let mut u = p.x - floor(p.x);
        let mut v = p.y - floor(p.y);
        let mut w = p.z - floor(p.z);
        u = u*u*(3.0-2.0*u);
        v = v*v*(3.0-2.0*v);
        w = w*w*(3.0-2.0*w);
        let i = floor(p.x) as i32;
        let j = floor(p.y) as i32;
        let k = floor(p.z) as i32;
        let mut c=vec![0.0;8];
        for i1 in 0..2{
            for j1 in 0..2{
                for k1 in 0..2{
                    c[(i1*4+j1*2+k1) as usize]=self.ranfloat[((self.perm_x[((i+i1)&255)as usize]) ^ (self.perm_y[((j+j1)&255) as usize]) ^ (self.perm_z[((k+k1)&255) as usize])) as usize];
                }
            }
        }
        return  trilinear_interp(&c, u, v, w);
    }
}
//     public:
//         perlin() {
//             ranfloat = new double[point_count];
//             for (int i = 0; i < point_count; ++i) {
//                 ranfloat[i] = random_double();
//             }

//             perm_x = perlin_generate_perm();
//             perm_y = perlin_generate_perm();
//             perm_z = perlin_generate_perm();
//         }

//         ~perlin() {
//             delete[] ranfloat;
//             delete[] perm_x;
//             delete[] perm_y;
//             delete[] perm_z;
//         }

//         double noise(const point3& p) const {
//             auto u = p.x() - floor(p.x());
//             auto v = p.y() - floor(p.y());
//             auto w = p.z() - floor(p.z());

//             auto i = static_cast<int>(4*p.x()) & 255;
//             auto j = static_cast<int>(4*p.y()) & 255;
//             auto k = static_cast<int>(4*p.z()) & 255;

//             return ranfloat[perm_x[i] ^ perm_y[j] ^ perm_z[k]];
//         }

//     private:


//         static int* perlin_generate_perm() {
//             auto p = new int[point_count];

//             for (int i = 0; i < perlin::point_count; i++)
//                 p[i] = i;

//             permute(p, point_count);

//             return p;
//         }

//         static void permute(int* p, int n) {
//             for (int i = n-1; i > 0; i--) {
//                 int target = random_int(0, i);
//                 int tmp = p[i];
//                 p[i] = p[target];
//                 p[target] = tmp;
//             }
//         }
// };
