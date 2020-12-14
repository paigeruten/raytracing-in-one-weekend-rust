use crate::{Color, Material, Ray, Vec3};
use crate::material::ScatterResult;
use crate::object::HitRecord;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected = r.direction().unit_vector().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some(ScatterResult { scattered, attenuation })
        } else {
            None
        }
    }
}
