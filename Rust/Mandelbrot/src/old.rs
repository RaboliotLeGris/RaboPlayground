//extern crate minifb;
extern crate image;

pub mod displays {
    mod core;
    mod fs;
//    mod live;
}

//use minifb::{Key, Window, WindowOptions};

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

const X1: f32 = -2.1;
const X2: f32 = 0.6;
const Y1: f32 = -1.2;
const Y2: f32 = 1.2;

const ZOOM: f32 = 600.0;
const MAX_ITERATION: i32 = 150;

const ALPHA: u32 = 255;
const RED: u32 = 255;
const BLUE: u32 = 255;
const GREEN: u32 = 255;

fn main() {
    let image_x: usize = ((X2 as i32 - X1 as i32) * ZOOM as i32) as usize;
    let image_y: usize = ((Y2 as i32 - Y1 as i32) * ZOOM as i32) as usize;

    println!("[INFO] Creating a window of: {}x{}", image_x, image_y);
    let mut buffer: Vec<u32> = vec![0; image_x * image_y];
    let mut img: RgbImage = ImageBuffer::new(image_x as u32, image_y as u32);

    for x in 0..image_x {
        for y in 0..image_y {
            let c_r: f32 = x as i32 as f32 / ZOOM + X1;
            let c_i: f32 = y as i32 as f32 / ZOOM + Y1;
            let mut z_r: f32 = 0.0;
            let mut z_i: f32 = 0.0;
            let mut i: i32 = 0;

            while {
                let tmp = z_r;
                z_r = z_r * z_r - z_i * z_i + c_r;
                z_i = 2.0 * tmp * z_i + c_i;
                i = i + 1;

                z_r * z_r + z_i * z_i < 4.0 && i < MAX_ITERATION
            } {}

            if i == MAX_ITERATION {
                img.put_pixel(x as u32, y as u32, image::Rgb([RED as u8, BLUE as u8, GREEN as u8]))
            }
        }
    }
    img.save("mandelbrot.png").unwrap();
}

