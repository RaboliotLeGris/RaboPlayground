use std::error::Error;
use std::fmt;
use std::fmt::{
    Display,
    Formatter,
};

fn main() {
    println!("error handling\n\n");

    // Options
    let some = some_options(true);
    println!("Options {:?}", some);
    let none = some_options(false);
    println!("Options {:?}\n\n", none);

    // Results
    let ok = some_result(true);
    println!("Result {:?}", ok);
    let err = some_result(false);
    println!("Result {:?}\n\n", err);
}

fn some_result(success: bool) -> Result<String, CustomError> {
    if success {
        return Ok(String::from("Success"))
    }
    Err(CustomError::SomeError(String::from("Some error")))
}

fn some_options(success: bool) -> Option<String> {
    if success {
        return Some(String::from("Some string"));
    }
    None
}

// Custom error definition: 
#[derive(Debug)]
enum CustomError {
    SomeError(String),
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::SomeError(s) => write!(f, "Custom error messag :  {}", s),
        }
    }
}