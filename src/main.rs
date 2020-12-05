mod color;
mod vec3;

use crate::vec3::Color;
use crate::color::write_color;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut stdout = std::io::stdout();

    for y in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", y);
        for x in 0..image_width {
            let pixel_color = Color::new(
                x as f64 / (image_width-1) as f64,
                y as f64 / (image_height-1) as f64,
                0.25,
            );
            write_color(&mut stdout, pixel_color);
        }
    }

    eprintln!("\nDone.");
}
