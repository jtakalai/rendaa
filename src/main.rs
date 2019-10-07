extern crate image;

use image::{ImageBuffer, Rgb};
use std::env;
use std::cmp::min;

fn main() {
    let args: Vec<String> = env::args().collect();
    let width: u32 = (&args[1]).parse().expect("Please give width");
    let height: u32 = (&args[2]).parse().expect("Please give height");

    println!("Drawing {} x {} image", width, height);

    let mut img = ImageBuffer::new(width, height);

    let size = min(width, height);

    for (px, py, pixel) in img.enumerate_pixels_mut() {
        let x = (px as f64 / size as f64) - 0.5;
        let y = (py as f64 / size as f64) - 0.5;
        let a = y.atan2(x);
        let r = (x * x + y * y).sqrt();
        let z = (1.0 - r * r).sqrt();
        let b = z.atan2(y);
        let c = z.atan2(x);

        let ruutu = ((7.0 * b) as u8 ^ (7.0 * c) as u8) % 2;

        let (col_r, col_g, col_b) = if r < 0.4 {
            (255, ruutu * 155 + 100, ruutu * 155 + 100)
        } else {
            (255, 100 + ((a * 7.0) as u8 % 2) * 155, 100)
        };
        //print!("({}, {} -> {}{}{}) ", px, py, r, g, b);
        *pixel = Rgb([col_r, col_g, col_b])
    }

    img.save("test.png").expect("Saving failed ):");
}
