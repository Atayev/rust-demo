struct Demo<'a> {
    field: &'a str,
} // single &str dont allowed to be used in struct, need to use lifetime <'a>, to ensure that the reference is valid for the lifetime of the struct

impl<'a> Demo<'a> {
    fn get_field(&self) -> &'a str {
        self.field
    }
} // lifetime using <'a> to ensure that the returned reference is valid for the lifetime of the struct

fn main() {
    // &' static this reference is valid for the lifetime of the program (app)

    let s = "hello"; // &'static "&str" by default lives for the lifetime of the program

    let demo = Demo { field: "hello" };
    let ret_val = demo.get_field();
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let greater_sum = compare_sum(&v1, &v2);

    for number in greater_sum.iter() {
        println!("{}", number);
    }
}

// fn sum<T>(numbers: &[T]) -> T
// where
//     T: std::marker::Copy + std::ops::Add<Output = T>,
// {
//     numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
// }

fn sum<'a, T>(numbers: &'a [T]) -> T
where
    T: std::iter::Sum<&'a T>,
{
    numbers.iter().sum()
}

fn compare_sum<'a>(vector1: &'a Vec<i32>, vector2: &'a Vec<i32>) -> &'a Vec<i32> {
    if sum(vector1) >= sum(vector2) {
        vector1
    } else {
        vector2
    }
} // lifetime using <'a > & 'a Vec<i32> to ensure that the returned reference is valid for the lifetime of the input references

fn do_smth(s: &str) -> &str {
    &s[..]
} // no need to use lifetime here, because the input reference is valid for the lifetime of the function, compiler will automatically infer the lifetime

//all it names lifetime ellision rules
