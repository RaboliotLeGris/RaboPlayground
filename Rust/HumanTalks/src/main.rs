mod samples {
    pub mod pattern_matching;
    pub mod result;
    pub mod structure;
    pub mod borrow;
    pub mod async_await;
}

use samples::*;
use crate::samples::structure::{Human, Mutant};

fn main() {
    println!("## -> Pattern Matching");
    pattern_matching::pattern_matching(String::from("coucou"));
    pattern_matching::pattern_matching(String::from("ca va"));
    pattern_matching::pattern_matching(String::from("no case"));

    println!("\n\n## -> Result");
    result::result();

    println!("\n\n## -> Structures");
    let human = structure::Human::create(String::from("Gérard"), 32);
    human.grow();

    let mutant: Human = structure::Mutant::create(String::from("Alphonse"), 34);
    println!("My mutant power is {}", mutant.power());

    println!("\n\n## -> Borrow checker");
    borrow::borrow();

    print!("\n\n## -> Async Await");
    async_await::run();
}
