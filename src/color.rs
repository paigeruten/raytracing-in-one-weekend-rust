use std::io::Write;

use crate::vec3::Color;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) {
    // Write the translated [0,255] value of each color component.
    writeln!(
        out,
        "{} {} {}",
        (255.999 * pixel_color.x()) as u8,
        (255.999 * pixel_color.y()) as u8,
        (255.999 * pixel_color.z()) as u8,
    ).unwrap();
}
