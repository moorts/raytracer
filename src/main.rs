mod vector;
mod color;
mod ray;

use std::fs::File;
use std::io::prelude::*;
use std::write;

use vector::Color;
use vector::Vec3;
use vector::Point3;
use color::to_string;

use ray::Ray;


fn main() -> std::io::Result<()> {

    let v1 = Vec3::new(1f64, 2f64, 3f64);
    println!("{:?}", v1);

    let origin = Point3::new(0f64, 0f64, 0f64);
    let direction = Vec3::new(1f64, 1f64, 1f64);

    let ray = Ray::new(origin, direction);

    println!("{:?}", ray.at(-5f64));

    let mut file = File::create("image.ppm")?;

    let image_width = 256;
    let image_height = 256;

    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let r: f64 = i as f64 / (image_width-1) as f64;
            let g: f64 = j as f64 / (image_height-1) as f64;
            let b: f64 = 0.25;
            let c = Color { x: r, y: g, z: b };

            write!(file, "{}", to_string(&c))?;
            if i < 255 {
                write!(file, "  ")?;
            }
        }
        write!(file, "\n")?;
    }

    eprintln!("\nDone.");
    Ok(())
}
