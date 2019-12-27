mod displays {
    pub mod core;
    pub mod fs;
    pub mod live;
}

use std::env;

use displays::fs::Fs;
use displays::live::Live;
use displays::core::Display;

const DEFAULT_SIZE: usize = 500;

fn print_help() {
    println!("Usage:");
    println!("\t -l -> Live displays");
    println!("\t -f -> Save to filesystem");
    println!("\t WIDTH -> define width of mandelbrot");
    println!("\t HEIGHT -> define height of mandelbrot");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 4 {
        let first_args = match args.get(1) {
            Some(s) => s,
            _ => ""
        };
        match first_args.as_ref() {
            "-h" => print_help(),
            "-l" => Live::show(atoi(&args, 2), atoi(&args, 3)),
            "-f" => Fs::show(atoi(&args, 2), atoi(&args, 3)),
            _ => Live::show(atoi(&args, 1), atoi(&args, 2))
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