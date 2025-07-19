enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // first variant stores no data
    Move { x: i32, y: i32 },    // second variant stores named fields
    Write(String),              // third variant stores a single String
    ChangeColor(i32, i32, i32), // fourth variant stores three i32  integer values
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Message called");
    } // method to call on Message enum
} //

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    //... other states
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter variant can also hold a UsState
}

fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    route(localhost);
    // enum Option<T> {
    //     Some(T),
    //     None,
    // } // Option enum to handle cases where a value may or may not be present
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");

    let absent_number: Option<i32> = Option::None; // Option enum to represent absence of a value

    let a = 5;
    let b = Option::Some(6);
    // let sum = a + b; // This will not compile because `b` is an Option type, not an i32
    let sum = a + b.unwrap_or(0); // Using unwrap_or to provide a default value if `b` is None
    println!("Sum: {}", sum);
    // Using the Message enum
    let msg = Message::Write(String::from("Hello, world!"));
    msg.call(); // Calling the method on the Message enum
                // Using the IpAddr struct

    // Using the Coin enum
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::California));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(10);
    match some_value {
        Some(10) => println!("Value is 10"),
        _ => (),
    } // Matching on an Option value to check if it is Some(10)

    if let Some(10) = some_value {
        println!("Value is 10 using if let");
    } // Using if let to check if some_value is Some(10)
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4(a, b, c, d) => {
            println!("IPv4 address: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddrKind::V6(addr) => {
            println!("IPv6 address: {}", addr);
        }
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Found a penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
} // Function to add one to an Option<i32>, returning an Option<i32>
