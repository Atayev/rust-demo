use crate::prelude::*;
use rand::Rng; // mod is creating new scope
mod random_number;
use random_number::RandomNumber; // struct from another module also we can use "use crate::generator::random_number::RandomNumber;"

// if you need everything from other module u can write "*" use random_number::*;

pub fn generate() -> RandomNumber {
    super::shared(); // using main modules public functions or etc..
    let rand_num = rand::thread_rng().gen_range(LOW..=HIGH); // random number generator
    RandomNumber::new(rand_num)
} // need to add pub to give access outside

pub fn other_fn() {
    println!("fasdf")
}
