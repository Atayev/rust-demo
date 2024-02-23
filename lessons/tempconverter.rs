use std::io;

const C: f32 = 32.0;

fn c_to_f(c_temp: f32) -> f32 {
    (c_temp * (9.0 / 5.0)) + C // all the numbers are f32 9.0/5.0 if no ; then it is returned automatically
}

fn f_to_c(f_temp: f32) -> f32 {
    (f_temp - C) * (5.0 / 9.0) // all the numbers are f32 5.0/9.0 if no ; then it is returned automatically
}

fn convert(temp: f32, choice: u8) -> Option<f32> {
    // Option<f32> - can give a value or None
    match choice {
        1 => Some(c_to_f(temp)),
        2 => Some(f_to_c(temp)),
        _ => None,
    } // match is like switch case
}

fn main() {
    println!("Temprature converter. \n (1) C to F \n (2) F to C");

    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).unwrap();

    let n_choice = match user_choice.trim().parse::<u8>() {
        Ok(1) => 1,
        Ok(2) => 2,
        _ => 0,
    };

    println!("Enter the temprature: ");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).unwrap();
    let tempature: f32 = temp.trim().parse().expect("Please enter a valid number");

    match convert(tempature, n_choice) {
        Some(result) => println!("The converted temprature is: {result}"),
        None => println!("Invalid choice"),
    };
}
