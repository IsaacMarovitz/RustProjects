use num::complex::Complex;
use image::{RgbImage, Rgb};
use std::time::SystemTime;

fn mandelbrot(c: Complex<f64>, z: Complex<f64>, iterations: u8, max_iterations: u8) -> u8 {
    let z_new:Complex<f64> = z*z + c;

    if z_new.norm() > 2.0 || iterations >= max_iterations {
        return iterations;
    } else {
        let iterations = iterations + 1;
        return mandelbrot(c, z_new, iterations, max_iterations);
    }
}

fn main() {
    let x_size = 20000;
    let y_size = 20000;
    let max_iterations = 255;
    let x_offset = 4.0 / x_size as f64;
    let y_offset = 4.0 / y_size as f64;
    let mut img = RgbImage::new(x_size, y_size);
    let start_time = SystemTime::now();
    
    for x in 0..x_size {
        for y in 0..y_size {
            let c: Complex<f64> = Complex::new(x as f64*x_offset-2.0, y as f64*y_offset-2.0);
            let mandelbrot_num:u8 = mandelbrot(c, Complex::new(0.0, 0.0), 1, max_iterations);
            if mandelbrot_num == 1 || mandelbrot_num == 6 {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            } else if mandelbrot_num == 0 {
                img.put_pixel(x, y, Rgb([159, 0, 255]));
            } else if mandelbrot_num == 2 {
                img.put_pixel(x, y, Rgb([0, 178, 51]));
            } else if mandelbrot_num == 3 {
                img.put_pixel(x, y, Rgb([255, 237, 0]));
            } else if mandelbrot_num == 4 {
                img.put_pixel(x, y, Rgb([255, 63, 49]));
            } else if mandelbrot_num == 5 {
                img.put_pixel(x, y, Rgb([0, 205, 255]));
            } else if mandelbrot_num == 6 {
                img.put_pixel(x, y, Rgb([159, 0, 255]));
            } else if mandelbrot_num == 7 {
                img.put_pixel(x, y, Rgb([0, 178, 51]));
            } else if mandelbrot_num == 8 {
                img.put_pixel(x, y, Rgb([255, 237, 0]));
            } else if mandelbrot_num == 9 {
                img.put_pixel(x, y, Rgb([255, 63, 49]));
            } else if mandelbrot_num == 10 {
                img.put_pixel(x, y, Rgb([0, 205, 255]));
            } else if mandelbrot_num >= 11 && mandelbrot_num <= 254 {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            } else if mandelbrot_num > 254 {
                img.put_pixel(x, y, Rgb([0, 178, 51]));
            }
        }
    }
    img.save("Mandelbrot4.png").expect("Image failed to save.");
    println!("Finished Mandelbrot in {:?} seconds", start_time.elapsed());
}