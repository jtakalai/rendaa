extern crate image;

use image::{ImageBuffer, Rgb};
use std::env;
use std::cmp::min;

fn main() {
    let args: Vec<String> = env::args().collect();
    let width: u32 = (&args[1]).parse().expect("Please give width");
    let height: u32 = (&args[2]).parse().expect("Please give height");

    println!("Drawing {} x {} image", width, height);

    let mut img = ImageBuffer::new(100, 100);

    let size = min(width, height);

    for (px, py, pixel) in img.enumerate_pixels_mut() {
        let x = (px as f64 / size as f64) - 0.5;
        let y = (py as f64 / size as f64) - 0.5;
        let r = x as u8;
        let g = y as u8;
        let b = 0 as u8;
        *pixel = Rgb([r, g, b])
    }

    img.save("test.png").expect("Saving failed ):");
}
