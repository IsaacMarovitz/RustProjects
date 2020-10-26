use num::complex::Complex;
use image::{RgbImage, Rgb};
use std::time::SystemTime;
use std::io;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use crossterm::{ExecutableCommand, terminal};

fn input(message: &str, failure_message: &str) -> u32 {
    let mut input_received = false;
    let mut return_int = 0;

    while !input_received {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input!");
        input = input.trim().to_string();
        match input.parse::<u32>() {
            Ok(n) => {
                return_int = n; 
                input_received = true;
            },
            Err(_) => {
                println!("{}", failure_message);
            },
        }
    }
    return_int
}

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
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    let x_size = input("Input x size: ", "Only input intergers!");
    let y_size = input("Input y size: ", "Only input intergers!");
    let max_iterations = 255;
    let x_offset = 4.0 / x_size as f64;
    let y_offset = 4.0 / y_size as f64;
    let mut img = RgbImage::new(x_size, y_size);
    let start_time = SystemTime::now();
    let pb = ProgressBar::new(x_size as u64);
    
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    println!("Rendering a {:?} x {:?} image of the Mandelbrot Set", x_size, y_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("=>-"));
    
    for x in 0..x_size {
        pb.set_position(x as u64);
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
    pb.finish();
    println!("Finished Mandelbrot in {:.1} seconds", start_time.elapsed().unwrap().as_secs_f32());
}