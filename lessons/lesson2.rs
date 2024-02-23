fn main() {
    println!("Hello, world!");
    let b = -3;
    let res = demo(5, b); // statement(demo func is expression)

    let result = loop {
        println!("Hello, world!");
        let mut counter: i32 = 0;
        if (counter == 10) {
            break counter * 2;
        }

        while counter > 100 {
            counter += 1;
        }

        let a: [i32; 5] = [1, 2, 3, 4, 5];

        for el in a {
            println!("The value is: {el}");
        }

        break 5; // exit the loop, if add something to break for ex. break 5; then it will return 5
    };
}

fn demo(a: u8, b: i32) -> i32 {
    println!("Hello, world! {a}");

    // if (a > 5) {
    //     a * 2
    // } else {
    //     a * 3
    // }

    return b * 2; // b*2 without ; is also valid
}

//statement - perform some action and do not return a value
//expression - evaluate to a resulting value
