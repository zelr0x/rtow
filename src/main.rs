#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]

use std::io::{self, Write};

use ray::Ray;
use hit::{Hit};
use sphere::Sphere;

use crate::{color::Color, point::Point3, vec3::Vec3};

pub mod color;
pub mod point;
pub mod ray;
pub mod hit;
mod sphere;
mod vec3;

fn main() -> io::Result<()> {
    // Image.
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i64 = 400;
    let image_height: i64 = (image_width as f64 / aspect_ratio) as i64;

    // Camera.
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        &origin - &horizontal / 2. - &vertical / 2. - Vec3::new(0., 0., focal_length);

    // Render.
    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        io::stderr().write(format!("\rScanlines remaining: {} ", j).as_bytes())?;
        io::stderr().flush()?;
        for i in 0..image_width {
            let u = (i as f64) / (image_width - 1) as f64;
            let v = (j as f64) / (image_height - 1) as f64;
            let r = Ray::new(
                &origin,
                &(&lower_left_corner + u * &horizontal + v * &vertical - &origin),
            );
            let color = ray_color(&r);
            println!("{}", &color);
        }
    }
    eprintln!("\nDone.");
    Ok(())
}

fn ray_color(r: &Ray) -> Color {
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let hit_res = sphere.hit(&r, f64::MIN, f64::MAX);
    let t = hit_res.map_or(-1.0, |x| x.t());
    if t > 0.0 {
        let n = Vec3::unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
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
