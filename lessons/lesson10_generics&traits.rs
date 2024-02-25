#![warn(clippy::all, clippy::pedantic)]

use std::{env::current_dir, process::Output};

#[derive(Debug)]
struct Employee<T, U> {
    // if everything in struct is of same type, we can use single  type <T>

    // if we want to use different types, we can use multiple types <T,U>
    age: T,
    salary: U,
    tax: U,
}
// impl<T, U> Employee<T, U> er : consider restricting type parameter `U`: `: std::ops::Mul` {

impl<T, U: std::marker::Copy + std::ops::Sub<Output = U> + std::ops::Mul<Output = U>>
    Employee<T, U>
{
    fn salary_after_tax(&self) -> U {
        self.salary - (self.salary * self.tax)
    }
}

trait Drive {
    fn drive(&self) -> bool;
}

struct Car {
    gas: u32,
}

impl Drive for Car {
    fn drive(&self) -> bool {
        self.gas > 0
    }
}

struct ElectroCar {
    battery: u32,
}
impl Drive for ElectroCar {
    fn drive(&self) -> bool {
        self.battery > 0
    }
}

fn car_info<T: Drive>(car: &T) {
    // generic function with trait Drive now function can accept any type that implements Drive trait
    println!("Car can drive {}", car.drive());
}

fn main() {
    let emp1 = Employee {
        age: 25,
        salary: 1000.0,
        tax: 0.1,
    };

    let salary = emp1.salary_after_tax();

    println!("Salary after tax: {salary}");
    let car = Car { gas: 10 };
    // let electro_car = ElectroCar { battery: 100 };

    car_info(&car);

    let v = vec![1, 2, 3];

    let _a = v.get(0); // Generic Option<&T>

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let number_list2: Vec<i8> = vec![1, 2, 3, 4, 5];

    let res = sum(&number_list);
    let res2 = sum(&number_list2);
}

// fn sum<T: std::marker::Copy + std::ops::Add<Output = T>>(numbers: &[T]) -> T {
//     // generic function with type T
//     // we can use multiple traits with + operator
//     // To allow copy trait we can use std::marker::Copy
//     // To allow add trait we can use std::ops::Add
//     numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
// }

fn sum<T>(numbers: &[T]) -> T
where
    T: std::marker::Copy + std::ops::Add<Output = T>,
{
    numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
} // same as above function but using where clause

// fn sum2(numbers: &[i32]) -> i32 {
//     numbers.iter().sum()
// }
