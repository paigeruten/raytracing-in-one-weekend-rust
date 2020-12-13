use std::io::Write;

use crate::util::clamp;
use crate::vec3::Color;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color, samples_per_pixel: i32) {
    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;

    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;

    // Write the translated [0,255] value of each color component.
    writeln!(
        out,
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ).unwrap();
}
