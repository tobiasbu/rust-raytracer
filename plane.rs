use vec3::Vec3;
use ray::Ray;
use material::Material;
use prim::Prim;
use intersection::Intersection;
use std::f64::INFINITY;

pub struct Plane {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub material: Box<Material>
}

impl Prim for Plane {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>> {
        let n = Vec3 {x: self.a, y: self.b, z: self.c};
        let nrd = n.dot(&ray.direction);
        let nro = n.dot(&ray.origin);
        let t = (-self.d - nro) / nrd;

        if t < t_min || t > t_max {
            None
        } else {
            let intersection_point = ray.origin + ray.direction.scale(t);

            Some(Intersection {
                n: n,
                t: t,
                position: intersection_point,
                material: &'a self.material
            })
        }
    }
}