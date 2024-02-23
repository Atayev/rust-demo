#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)] //trait
struct Car {
    brand: String,
    max_speed: u16,
    max_gas: f32,
    curr_gas: f32,
    gas_consum: f32,
}

impl Car {
    fn new(brand: &str, max_speed: u16, max_gas: f32, gas_consum: f32) -> Self {
        Self {
            brand: String::from(brand),
            max_speed,
            max_gas,
            curr_gas: max_gas,
            gas_consum,
        }
    }

    fn drive(&mut self, distance: f32) {
        let total_gas_needed = distance * self.gas_per_km();

        if total_gas_needed > self.curr_gas {
            println!("Not enough gas to drive that far");
        } else {
            self.curr_gas -= total_gas_needed;
            println!("Driving {} km", distance);
        }
    }

    fn gas_per_km(&self) -> f32 {
        self.gas_consum / 100.0
    }

    fn faster(&self, other: &Car) -> bool {
        self.max_speed > other.max_speed
    }
} // struct methods

struct Color(u8, u8, u8); //struct without fields names

struct Person; // struct without fields

fn main() {
    let mut my_car = Car {
        brand: String::from("Toyota"),
        max_speed: 200,
        max_gas: 50.0,
        curr_gas: 50.0,
        gas_consum: 0.5,
    };

    println!(
        "My car is a {} and it can go up to {} km/h",
        my_car.brand, my_car.max_speed
    );

    println!("{my_car:#?}");

    let distance = 40.0;
    my_car.drive(distance);

    let car1 = Car::new("Volvo", 200, 50.0, 0.5);

    println!("{car1:#?}");
    let car2 = Car {
        curr_gas: 0.0,
        ..car1
    }; // struct update syntax like js but with . .

    println!("{car2:#?}");
}
