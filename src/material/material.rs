use crate::object::HitRecord;
use crate::{Color, Ray};

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}
