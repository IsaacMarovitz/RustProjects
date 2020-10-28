use image::{RgbImage, Rgb};
use std::time::SystemTime;
use std::io;
use std::io::Write;
use std::f64;
use indicatif::{ProgressBar, ProgressStyle};
use crossterm::{ExecutableCommand, terminal};
use std::process::Command;

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

fn julia(c: [f64; 2], z: [f64; 2], escape_radius: u32, max_iterations: u32) -> u32 {
    let mut iterations: u32 = 1;
    let mut zx: f64 = z[0];
    let mut zy: f64 = z[1];

    while zx * zx + zy * zy < (escape_radius * escape_radius) as f64 && iterations < max_iterations {
        let xtemp = zx * zx - zy * zy;
        zy = 2.0 as f64 * zx * zy + c[1];
        zx = xtemp as f64 + c[0];

        iterations += 1;
    }

    return iterations;
}

fn v(m1: f32, m2: f32, hue: f32) -> f32 {
    let hue = hue % 1.0;
    if hue < (1.0/6.0) {
        return m1 + (m2-m1) * hue * 6.0;
    }
    if hue < 0.5 {
        return m2;
    }
    if hue < (2.0/3.0) {
        return m1 + (m2-m1) * ((2.0/3.0) - hue) * 6.0
    }
    return m1
}

fn hsl_to_rgb(h: u32, s: f32, l: f32) -> [u8; 3] {
    let h: f32 = h as f32 / 360.0;
    let s: f32 = s / 100.0;
    let l: f32 = l / 100.0;

    if s == 0.0 {
        return [(l*255.0) as u8, (l*255.0) as u8, (l*255.0) as u8];
    } else {
        if l <= 0.5 {
            let m2 = l * (1.0 + s);
            let m1: f32 = 2.0*l - m2;
            return [(v(m1, m2, h + (1.0/3.0)) * 255.0) as u8, (v(m1, m2, h) * 255.0) as u8, (v(m1, m2, h - (1.0/3.0)) * 255.0) as u8];
        } else {
            let m2 = l + s - (l * s);
            let m1: f32 = 2.0*l - m2;
            return [(v(m1, m2, h + (1.0/3.0)) * 255.0) as u8, (v(m1, m2, h) * 255.0) as u8, (v(m1, m2, h - (1.0/3.0)) * 255.0) as u8];
        }
    }
}

fn main() {
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    let x_size = input("Input x size: ", "Only input intergers!");
    let y_size = input("Input y size: ", "Only input intergers!");
    let x_limits: [f64; 2] = [-2.0, 2.0];
    let y_limits: [f64; 2] = [-2.0, 2.0];
    let escape_radius = 10;
    let max_iterations = 255;
    let mut img = RgbImage::new(x_size, y_size);
    let start_time = SystemTime::now();

    let max: f64 = f64::consts::PI * 2 as f64;
    let step = 0.001;
    let mut current: f64 = 0.0;
    let mut i: u32 = 0;
    let tpb = ProgressBar::new((max/step) as u64);
    
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    println!("Rendering a {:?} x {:?} animation of the Julia Set", x_size, y_size);
    tpb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("=>-"));
    
    while current < max {
        tpb.set_position(i as u64);
        for y in 0..y_size {
            let cy = y as f64 * (y_limits[1] - y_limits[0]) / y_size as f64 + y_limits[0];
            for x in 0..x_size {
                let cx = x as f64 * (x_limits[1] - x_limits[0]) / x_size as f64 + x_limits[0];
                let julia_num: u32 = julia([current.cos(), current.sin()], [cx, cy], escape_radius, max_iterations);
                img.put_pixel(x, y, Rgb(hsl_to_rgb((julia_num as f32*15.0/255.0*360.0) as u32, 100.0, 50.0)));
            }
        }
        img.save("./imgs/".to_owned() + &i.to_string() + ".png").expect("Image failed to save.");
        i += 1;
        current = current + step;
    }
    tpb.finish();
    println!("Finished generating frames");

    match Command::new("ffmpeg")
            .args(&["-framerate", "60", "-i", "./%d.png", "-pix_fmt", "yuv420p", "julia.mp4", "-y"])
            .current_dir("./imgs") 
            .output() {
        Ok(_) => {
            println!("Finished generating video");
            println!("Finished Julia Set in {:.1} seconds", start_time.elapsed().unwrap().as_secs_f32());
        },
        Err(_) => {
            println!("Failed to make video!");
        },
    } 
}