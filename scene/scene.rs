use geometry::Prim;
use light::Light;
use vec3::Vec3;

pub struct Scene {
    pub lights: Vec<Box<Light>>,
    pub prims: Vec<Box<Prim>>,
    pub background: Vec3
}