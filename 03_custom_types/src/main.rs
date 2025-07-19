struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)] // Derive the Debug trait to enable printing the struct with {:?}
struct Rectangle {
    width: u32,
    height: u32,
}
// implentation block for the Rectangle struct:
// this is where we define methods and associated functions for the Rectangle struct
impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
} // associated function to create a square rectangle
  // associated functions are functions that are associated with a type, but do not take `self`
  // they are called using the type name, like `Rectangle::square(size)`
  // they are similar to static methods in other languages, but in Rust, they are called associated functions
  // associated functions are often used to create new instances of a type, like in this case where we create a square rectangle with a given size.
  // they can also be used to define utility functions that are related to the type, but do not operate on an instance of the type.
  // this is useful for creating new instances of a type without having to create a new instance first and then call a method on it.
  // this is a common pattern in Rust, where you define an associated function to create a new instance of a type,
  // and then use that function to create new instances of the type.

fn main() {
    let mut user1 = User {
        username: String::from("abc"),
        email: String::from("abc@gmail.com   "),
        active: true,
        sign_in_count: 1,
    };
    //let name = user1.username;
    user1.username = String::from("pili");

    let user2 = build_user(String::from("qwe@gmail.com"), String::from("qwe"));

    let user3 = User {
        email: String::from("asd@gmail.com"),
        username: String::from("asd"),
        ..user2 // using struct update syntax to copy fields from user2
    };
    println!(
        "User1: {}, {}, {}, {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
    println!("User2: {}, {}", user2.username, user2.email);
    println!("User3: {}, {}", user3.username, user3.email);

    //tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // the fields of a tuple struct can be accessed using dot notation
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin pt: ({}, {}, {})", origin.0, origin.1, origin.2);
    // Tuple structs are a way to create custom data types that can hold a fixed number of values, similar to tuples, but with a name that makes them more descriptive.
    // They can be useful when you want to create a type that has a specific meaning, but you don't need named fields like in a regular struct.
    // the difference between a tuple struct and a regular tuple is that a tuple struct has a name, which can make the code more readable.
    let width1 = 30;
    let height1 = 50;
    println!(
        "the area of a rectangle is: {} square pixels.",
        area_basic(width1, height1)
    );

    let rect_tuple = (30, 50); // using a tuple to represent dimensions

    println!(
        "the area of a rectangle is: {} square pixels.",
        area_with_tuple(rect_tuple)
    );
    let rect_struct = Rectangle {
        width: 30,
        height: 50,
    }; // using a struct to represent dimensions

    let rect_struct2 = Rectangle {
        width: 20,
        height: 40,
    }; // another rectangle with different dimensions

    let rect_struct3 = Rectangle {
        width: 40,
        height: 50,
    }; // another rectangle with same dimensions as rect_struct
    println!(
        "the area of a rectangle is: {} square pixels.",
        area_with_struct_and_tuple(&rect_struct)
    );
    let rect_struct_square = Rectangle::square(30); // using the associated function to create a square rectangle
    println!(
        "the area of a square rectangle is: {} square pixels.",
        area_with_struct_and_tuple(&rect_struct_square)
    );

    println!("(with the debug trait) the  rect: {:#?}", rect_struct);
    // the `{:?}` format specifier is used to print the struct with the Debug trait, which is derived above
    // this allows you to see the values of the fields of the struct in a readable format
    // you can also use {:#?} for pretty-printing, which will format the output with indentation and newlines for better readability.
    // the Debug trait is a built-in trait in Rust that allows you to format a value using the {:?} format specifier.
    // this is useful for debugging purposes, as it allows you to see the values of the fields of the struct in a readable format.
    // you can derive the Debug trait for a struct by adding `#[derive(Debug)]` above the struct definition.

    println!(
        "the area of a rectangle is: {} square pixels.",
        rect_struct.area() // calling the method defined in the Rectangle impl block
    ); // this is more readable than using a function that takes a struct as an argument

    // AUTOMATIC REFERENCING AND DEREFERENCING:
    // when you call a method on a struct, Rust automatically adds a reference to the struct
    // so you don't need to pass a reference to the struct explicitly.
    // this is because methods are defined with `&self`, which is a shorthand for `&Self`, where `Self` is the type of the struct.
    // this allows you to call methods on a struct without having to worry about references and dereferencing.

    println!(
        "Does rect_struct hold rect_struct2? {}",
        rect_struct.can_hold(&rect_struct2) // using the method defined in the Rectangle impl block
    );
    // the `&` before `rect_struct2` is used to pass a reference to the rectangle to the method.
    // this is because the method takes a reference to another rectangle as an argument.
    // this allows you to compare the dimensions of two rectangles without having to copy the entire rectangle.
    println!(
        "Does rect_struct hold rect_struct3? {}",
        rect_struct.can_hold(&rect_struct3)
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username, // or simply `email, username`
        active: true,
        sign_in_count: 1,
    }
}

fn area_basic(width: u32, height: u32) -> u32 {
    width * height
} // more readable version using tuple structs below
fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
} // more readable version using a struct below
fn area_with_struct_and_tuple(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
