#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]

use std::io::{self, Write};

use crate::color::Color;

pub mod color;
pub mod point;
pub mod ray;
mod vec3;

fn main() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for j in (0..=image_height - 1).rev() {
        io::stderr().write(format!("\rScanlines remaining: {}", j).as_bytes())?;
        io::stderr().flush()?;
        for i in 0..image_width {
            let r = i as f64 / (image_width as f64 - 1.0);
            let g = j as f64 / (image_height as f64 - 1.0);
            let b = 0.25;
            println!("{}", &Color::new(r, g, b));
        }
    }
    io::stderr().write(b"\nDone.\n")?;
    Ok(())
}
