fn main() {
    let output = str_data();
    println!("{output}");
}

fn str_data() -> &'static str {
    "Hello, world!"
}
fn boom() {
    eprintln!("error"); // print to stderr instead of stdout - useful for debugging.
    panic!("Boom!");
}

fn result(a: u8) -> Result<u8, String> {
    if a == 42 {
        Ok(a)
    } else {
        Err("Not 42".to_string())
    }
}

#[cfg(test)] // config for tests (unit)
mod tests {
    // use crate::str_data; // import the function
    use super::*; // import everything from the parent module.
    use pretty_assertions::assert_eq; // import the assert_eq macro from the pretty_assertions crate.

    #[test]

    fn it_panics_when_a_is_too_large() -> Result<(), String> {
        let res = result(190);

        if res.is_err() {
            Err("Got an error".into())
        } else {
            Ok(())
        }
    }

    #[test]
    // #[should_panic] // attribute for tests if it always panics -- not recommended
    #[should_panic(expected = "Boom!")] // attribute for tests if it says it should panic and the expected message.
    fn it_always_panics() {
        boom();
    }

    #[test]

    fn it_returns_str() {
        assert_eq!(str_data(), "Hello, world!", "Innvalid string") // you can pass 3rd argument to assert_eq to display a custom message.
    }

    #[test] // attribute for tests
    #[ignore] // if this test should be ignored (expensive tests, etc.)
              // by default fn names starts with it_
    fn it_returns_true() {
        // assert!(true); // assert macro - checks if the expression is true. If false test will fail.
        //  assert_eq!(2 + 2, 4); // assert_eq (equal) macro - checks if the two expressions are equal. If not test will fail.

        // assert_ne!(2 + 2, 5); // assert_ne (not equal) macro - checks if the two expressions are not equal. If they are equal test will fail.
    }
}

// cargo test -- test-threads=1 // run tests in paralel
// cargo test -- --show-output // show the output of the tests
// cargo test -- testname // run a specific test
// cargo test -- --ignored // run only the ignored tests
// cargo test -- --include-ignored // run all the tests including the ignored ones

// auto tests
// unit tests - tests for individual functions or scenarios.
// integration tests - tests for the entire program. all the parts working together.

// system tests - tests for the entire program. all the parts working together. testing the program in a different real environment.

// e2e tests - tests for the entire program. all the parts working together. testing the program in a different real environment. using the program as a user would.

// smoke tests - tests for the entire program. all the parts working together. testing the program in a different real environment. using the program as a user would. testing the most important features.
// tdd - test driven development - write tests first, then write the code to make the tests pass.

// red, green, refactor - write a failing test, write the code to make the test pass, refactor the code.
