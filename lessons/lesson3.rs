#![warn(clippy::all, clippy::pedantic)]

// stack - int , float, char, bool, pointer , LIFO
// heap - str
// no garbage collector
// fn main() {
//     //scope
//     //ownership
//     let a = 32; //stack, value 32 is stored in a, a is owner of 32
//     demo(a);

//     println!("Hello, world! {}", a);

//     let s = String::from("hello"); // heap
// }

// fn demo(a2: i32) -> i32 {
//     a2 * 2
// }

// fn main() {
//     let mut a = 42;
//     let b = a;

//     // if a changes b not changes

//     // borrowing
//     let s1 = String::from("helloo");
//     // let s2 = s1; //if use .clone() it copies //by default move pointer s1 moves to s2 and s1 deletes

//     // demo_str(s1); // moved
//     // same memory pointer in heap
//     // there is no s1 after you assinged it to s2

//     let l = demo_str(&s1); //borrowing using & pointer
//                            // primitive types not moving they are copying
//     println!("{s1}") // error borrow of moved value s1
// }

// fn demo_str(s3: &String) -> usize {
//     //...

//     s3.len()
// } // & borrowing, if add mut keyword you can change string that borrowed

// fn main() {
//     let s1 = String::from("helo");

//     let _s2 = &s1[0..3]; // slice &str slice type

//     let _str = "string"; //literal is slice too
// }

use std::io;

fn main() {
    let mut u_choice = String::new();

    io::stdin().read_line(&mut u_choice).unwrap();

    let n_choice = process(&u_choice);

    println!("Number : {n_choice}");
    println!("String : {u_choice}");
}

fn process(str: &str) -> u8 {
    str.trim().parse::<u8>().expect("Please type a number!")
} // u can write &String like &str too, it means that you can use either slice either String type
