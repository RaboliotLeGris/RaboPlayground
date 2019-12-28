pub mod displays;
pub mod fractals ;

use std::env;

use displays::fs::Fs;
use displays::live::Live;
use displays::traits::Display;
use fractals::mandelbrot::Mandelbrot;
use fractals::julia::Julia;

const DEFAULT_SIZE: usize = 1000;

fn print_help() {
    println!("Usage:");
    println!("\t -l -> Live displays");
    println!("\t -f -> Save to filesystem");
    println!("\t WIDTH -> define width of mandelbrot");
    println!("\t HEIGHT -> define height of mandelbrot");
}

fn main() {
    let fractal_generator = Julia::new();

    let args: Vec<String> = env::args().collect();
    if args.len() <= 4 {
        let first_args = match args.get(1) {
            Some(s) => s,
            _ => ""
        };
        match first_args.as_ref() {
            "-h" => print_help(),
            "-l" => Live::show(atoi(&args, 2), atoi(&args, 3), fractal_generator),
            "-f" => Fs::show(atoi(&args, 2), atoi(&args, 3), fractal_generator),
            _ => Live::show(atoi(&args, 1), atoi(&args, 2), fractal_generator)
        };
    } else {
        print_help();
    }
}

fn atoi(args: &Vec<String>, index: u32) -> usize {
    match args.get(index as usize) {
        None => DEFAULT_SIZE,
        Some(num) => num.parse().unwrap_or(DEFAULT_SIZE)
    }
}