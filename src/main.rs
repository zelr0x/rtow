#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]

#[macro_use]
extern crate lazy_static;

use std::{io::{self, Write}, rc::Rc};

use crate::{
    camera::Camera,
    color::Color,
    hit::{Hit, HitList},
    point::Point3,
    ray::Ray,
    sphere::Sphere,
};

pub mod camera;
pub mod color;
pub mod point;
pub mod ray;
pub mod hit;
pub mod util;
mod sphere;
mod vec3;

fn main() -> io::Result<()> {
    // Image.
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i64 = 400;
    let image_height: i64 = (image_width as f64 / aspect_ratio) as i64;

    // World.
    let mut world = HitList::with_capacity(2);
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera.
    let camera = Camera::new(aspect_ratio);

    // Render.
    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        io::stderr().write(format!("\rScanlines remaining: {} ", j).as_bytes())?;
        io::stderr().flush()?;
        for i in 0..image_width {
            let u = (i as f64) / (image_width - 1) as f64;
            let v = (j as f64) / (image_height - 1) as f64;
            let r = camera.ray(u, v);
            let color = ray_color(&r, &world);
            println!("{}", &color);
        }
    }
    eprintln!("\nDone.");
    Ok(())
}

fn ray_color(r: &Ray, world: &HitList) -> Color {
    if let Some(hd) = world.hit(&r, 0.0, f64::INFINITY) {
        return 0.5 * (hd.normal() + Color::new(1.0, 1.0, 1.0))
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn ppm() {
        assert!(render_ppm().is_ok())
    }

    fn render_ppm() -> io::Result<()> {
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
}
