#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
enum OrderStatus {
    Pending { created_at: String }, // you can named add data to the enum variant {},
    Shipped,
    Delivered(String), // you can add unnamed data to the enum variant ()
}
#[derive(Debug)]
struct Order {
    customer: String,
    status: OrderStatus,
}

impl OrderStatus {
    fn info(&self) {
        match self {
            Self::Pending { created_at } => {
                println!("The order was created at: {created_at}");
            }
            Self::Shipped => {
                println!("The order was shipped");
            }
            Self::Delivered(date) => {
                println!("The order was delivered at: {date}");
            }
        };
    }
}

fn main() {
    let status = OrderStatus::Pending {
        created_at: String::from("2021-01-01"),
    };
    show_status(&status);

    let order = Order {
        customer: String::from("John Doe"),
        status,
    };
    println!("The order is: {order:#?}");

    let status2: OrderStatus = OrderStatus::Delivered(String::from("2021-01-01")); // unnamed data

    let delivered = OrderStatus::Delivered(String::from("2021-01-01"));

    delivered.info();

    // Option
    let value: Option<i8> = Some(5); //some value
    let value2: Option<i8> = None; //no value

    //Option is a generic enum

    let res = value.unwrap(); // opens the option and returns the value
    let res2 = value2.unwrap_or(0); // opens the option and returns the value or the default value

    match value {
        Some(v) => {
            let result = v + 5;

            println!("The value is: {result}");
        }
        None => println!("There is no value"),
    }

    if let Some(a) = value {
        let res = a + 5;
        println!("The value is: {res}");
    } else {
        println!("There is no value");
    } // if let is a shortcut for match
}

fn show_status(status: &OrderStatus) {
    println!("The status is: {status:#?}");
}
