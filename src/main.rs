extern crate image;

use image::{ImageBuffer, Rgb};
use std::env;
use std::cmp::min;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { panic!("Usage: main.rs width height frames") }
    let width: u32 = (&args[1]).parse().expect("Please give width as command-line argument");
    let height: u32 = (&args[2]).parse().expect("Please give height as command-line argument");
    let frames: u32 = (&args[3]).parse().expect("Please give frame number as command-line argument");

    let mut img = ImageBuffer::new(width, height);

    let size = min(width, height);
    let dt = 3.1415926 / (frames as f64);
    let radius2 = 0.1;

    for i in 0..frames {
        let t = dt * i as f64;
        for (px, py, pixel) in img.enumerate_pixels_mut() {
            let x = (px as f64 / size as f64) - 0.5 + (t * 2.0).cos() * 0.2;
            let y = (py as f64 / size as f64) - 0.5 + (t * 4.0).sin() * 0.2;
            let a = y.atan2(x);
            let r2 = x * x + y * y;

            *pixel = if r2 < radius2 {
                let z = (1.0 - r2 / radius2 / 2.0).sqrt();
                let b = z.atan2(y);
                let c = z.atan2(x);
                let ruutu = ((7.0 * b) as u8 ^ (7.0 * c + t * 2.0) as u8) % 2;
                Rgb([255, ruutu * 155 + 100, ruutu * 155 + 100])
            } else {
                Rgb([255, 100 + ((a * 7.0 + t * 2.0) as u8 % 2) * 155, 100])
            }
        }
        let filename = format!("frames/frame{:0>3}.png", i);
        println!("Saving {} x {} image to {}", width, height, filename);
        img.save(filename).expect("Saving failed ):");
    }
}
