mod lambertian;
mod material;
mod metal;

pub use lambertian::Lambertian;
pub use material::{Material, ScatterResult};
pub use metal::Metal;
