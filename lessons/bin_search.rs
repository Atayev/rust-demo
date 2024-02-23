#![warn(clippy::all, clippy::pedantic)]

use std::cmp::Ordering;

fn main() {
    let arr: [i32; 8] = [-1, 2, 3, 4, 5, 32, 58, 134];
    let res = bin_search(&arr, -100);
    match res {
        Some((val, index)) => println!("The value {val} is at index {index}"),
        None => println!("The value is not in the array"),
    }
}

fn bin_search(arr: &[i32], desired_val: i32) -> Option<(i32, usize)> {
    let mut low_bound: usize = 0;
    let mut high_bound: usize = arr.len() - 1;
    let mut i: usize = 0;

    while low_bound <= high_bound {
        i += 1;
        let mid = (low_bound + high_bound) / 2;
        let mid_val = arr[mid];

        match mid_val.cmp(&desired_val) {
            Ordering::Equal => return Some((mid_val, mid)),
            Ordering::Greater => {
                high_bound = match mid.checked_sub(1) {
                    Some(res) => res,
                    _ => return None,
                }
            }
            // high_bound = mid - 1, // error overflow if mid is 0
            Ordering::Less => low_bound = mid + 1,
        } //cmp function compares , Ordering is for compareing
          // if mid_val == desired_val {
          //     return Some((mid_val, mid));
          // } else if desired_val > mid_val {
          //     low_bound = mid + 1;
          // } else {
          //     high_bound = mid - 1;
          // }
        println!("Step:{i}");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARR: [i32; 8] = [-1, 2, 3, 4, 5, 32, 58, 134];
    #[test]
    fn element_found() {
        assert_eq!((-1, 0), bin_search(&ARR, -1).unwrap());
    }

    #[test]
    fn element_not_found() {
        let res = bin_search(&ARR, 1234);
        assert!(res.is_none());
    }
}
