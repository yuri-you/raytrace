use crate::ray::Ray;
use crate::vec3::Vec3;
// #[derive(Clone, Debug, PartialEq)]
pub fn write_color(rays:Ray)->Vec3{
    Vec3::new(
        (rays.direction.x+512.0)*256.0/1024.0,
        (rays.direction.y+256.0)*256.0/512.0,
        128.0,
    )
}