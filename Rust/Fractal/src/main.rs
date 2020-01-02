pub mod displays;
pub mod fractals ;

use std::env;

use displays::{
    fs::Fs,
    live::Live,
    traits::Display
};

use crate::fractals::FractalFactory;

const DEFAULT_SIZE: usize = 1000;

fn print_help() {
    println!("Usage:");
    println!("\t -l -> Live displays");
    println!("\t -f -> Save to filesystem");
    println!("\t WIDTH -> define width of mandelbrot");
    println!("\t HEIGHT -> define height of mandelbrot");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 5 {
        let first_args = match args.get(1) {
            Some(s) => s,
            _ => ""
        };
        match first_args.as_ref() {
            "-h" => print_help(),
            "-l" => Live::show(atoi(&args, 3), atoi(&args, 3), FractalFactory::make(args.get(2))),
            "-f" => Fs::show(atoi(&args, 3), atoi(&args, 3), FractalFactory::make(args.get(2))),
            _ => Live::show(atoi(&args, 2), atoi(&args, 2), FractalFactory::make(args.get(1)))
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