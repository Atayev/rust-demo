fn main() {
    let arr: [i32; 8] = [-1, 2, 3, 4, 5, 32, 58, 134];

    let res = find_el(&arr, 32);

    match res {
        Some((index, val)) => println!("The value {val} is at index {index}"),
        None => println!("The value is not in the array"),
    }
}

fn find_el(arr: &[i32], el: i32) -> Option<(usize, i32)> {
    let mut i: usize = 0;

    while i < arr.len() {
        if arr[i] == el {
            return Some((i, el));
        }
        i += 1;
    }
    None
}
