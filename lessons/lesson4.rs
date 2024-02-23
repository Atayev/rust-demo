fn q2() {
    let x: u64 = 4_294_967_296;
    let y = x as u32; // y==0, can be loss - casting `u32` to `u64` may become silently lossy if you later change the type  `u64::from(y)`
    let c: u32 = x.try_into().expect("Fail!"); //

    if x == y as u64 {
        println!("x equals y");
    } else {
        println!("x does not equal y."); // true because u32 max number is 2**32-1 = 4294967295
    }
}

fn q() {
    // let mut counter = Wrapping(0u8); //Provides intentionally-wrapped arithmetic on `T`

    let mut counter: u8 = 0;
    loop {
        // counter += 1;
        // counter = Wrapping(1i8)
        counter = counter.checked_add(1).expect("fail!"); // if overflow return message Fail!
        println!("counter: {}", counter);
    } //overflow in development, infinite loop in release
}

fn call_me(n: u64, _: i32, c: u32) -> u64 {
    println!("{c}"); // c will be 4294967293 if -3 is casted to u32
    n
}

fn q8() {
    let one: i32 = 1;
    let c: i8 = -3;
    let n = call_me(one as _, 3, c as _); // use "as _" to convert a specific type in called fn
}
