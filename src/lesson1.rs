// const PI: f64 = 3.14; // constant variable
fn main() {
    let a: i32 = 5; //immutable variable

    let mut b: i32 = 8; //mutable variable - keyword - 'mut'

    b = b + 10; // mutates the value of b
    println!("Hello, world! {a} and {b}");

    //scalar types
    let c: i32 = 5; //signed integer i8, i16, i32, i64, i128
                    // i8 - 8 bit -128 to 128-1
                    // i16 - 16 bit - 2^15 to 2^15-1 1** for sign
                    // i32 - 32 bit - 2^31 to 2^31-1
                    // i64 - 64 bit - 2^63 to 2^63-1
                    // i128 - 128 bit - 2^127 to 2^127-1

    let hex: i32 = 0xff; //hexadecimal
                         //0o - octal
                         //0b - binary
                         //0x - hexadecimal

    let d: u32 = 5; //unsigned integer u8, u16, u32, u64, u128
                    // u8 - 8 bit - 0 to 255
                    // u16 - 16 bit - 0 to 65535
                    // u32 - 32 bit - 0 to 2^32-1
                    // u64 - 64 bit - 0 to 2^64-1
                    // u128 - 128 bit - 0 to 2^128-1

    let f: isize = 5; //depends on the architecture of the machine

    let g: f32 = 5.0; //floating point f32, f64
                      // f32 - single precision
                      // f64 - double precision - default

    // operations
    // + - * / % - arithmetic

    // boolean
    let h: bool = true; // true or false

    // Char
    let ch: char = 'a'; // single character only single quotes '' 4 bytes unicode

    // compound types
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple of 3 elements with different types
    let (x, y, z) = tup; // destructuring

    let tuple_int = tup.0; // accessing elements using index

    // Array
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // array of 5 elements of type i32 only same type [type; size] fixed size

    arr[3] = 32 // accessing elements using index
}
