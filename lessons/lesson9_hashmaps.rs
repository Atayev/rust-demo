use std::collections::HashMap;

fn main() {
    let mut a = HashMap::new();

    a.insert(String::from("demo"), 42);
    a.insert(String::from("key"), 43);
    // a.insert(String::from("key"), 32); // if second time same key appending it is rewrite the value

    let value = a.get(&String::from("key")).copied().unwrap_or(1); // copied() for borrowing

    a.entry(String::from("key")).or_insert(50); // if key is not present then insert the value

    for (k, v) in &a {
        println!("Key {k}: Value {v}");
    }

    println!("{a:#?}")
}
