#![warn(clippy::all, clippy::pedantic)]

// if u want to use all module u need to import it using keyword "use"

mod generator;

// if u want to add global using functions crates(libraries) you need to create prelude - it is mod too

mod prelude {
    pub use crate::generator::{generate, other_fn};
    // use generator::generate; // accessing function inside generator module
    pub use std::env;

    pub const LOW: u8 = 1;
    pub const HIGH: u8 = 150;
}

use prelude::*;
// u can use as keyword to change name of import u want for example (alias)

// use std::ascii as asc;

fn main() {
    other_fn();
    let random = generate(); // one way to use modules here is to use his name generator::funcname
    println!("random number {}", random.value);

    let args: Vec<String> = env::args().collect();

    println!("{args:#?}")
}

// we can create module here using keyword "mod"

// two types of creates binary and library crates
// binary is when default main.rs file exist
// library create starts from lib.rs they are connecting using cargo
// package is package of different crates

pub fn shared() {
    println!("asdf")
}
