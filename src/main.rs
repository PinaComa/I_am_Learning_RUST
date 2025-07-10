/* I am following Bogdan's videos */

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This line changes the value of x
    println!("The value of x is: {}", x);

    {
        let x = 10; // This shadows the outer x
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x after the inner scope is: {}", x);

    //integers
    //floating point numbers
    //boelans
    //characters
    //tuples
    let f: u8 = 255; // u8 can hold values from 0 to 255
                     //f = 256; // This line would cause a compile-time error because 256 is out of the range for u8
    println!("The value of f is: {}", f);
    let tup = ("lets ... ", 100_000);
    println!("The value of tup is: {:?}", tup);
    let (text, number) = tup; // Destructuring the tuple
    println!("Text: {}, Number: {}", text, number);
    println!("Text: {}, Number: {}", tup.0, tup.1); // Accessing tuple elements directly

    //arrays
    let arr = [1, 2, 3, 4, 5];
    println!("The first element of the array is: {}", arr[0]);

    println!("The length of the array is: {}", arr.len());
    println!("The array is: {:?}", arr); // Printing the entire array

    let y = sum_multiply(10, 20); // Call the function with parameters
    println!("The product of 10 and 20 is: {}", y);
    conditionals(); // Call the function to demonstrate conditionals
    loops(); // Call the function to demonstrate loops
}

fn sum_multiply(x: i32, y: i32) -> i32 {
    println!("****This is func = sum and multiply!****");
    println!("The sum of x and y is: {}", x + y);
    let product = x * y;
    return product; // Returning the product of x and y
}

fn conditionals() {
    println!("****This is the conditionals function!****");
    let number = 5;

    if number < 10 {
        println!("The number is less than 10");
    } else if number == 10 {
        println!("The number is equal to 10");
    } else {
        println!("The number is greater than 10");
    }

    let condition = true;
    let number = if condition { 5 } else { 10 }; // Using an if expression
    println!("The value of number is: {}", number);

    // Using a match statement
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
}
fn loops() {
    println!("****This is the loops function!****");
    let mut count = 0;

    let final_count = loop {
        println!("Count is: {}", count);
        count += 1;
        if count == 10 {
            break count; // breaks with value and assigns it
        }
    };
    // The final_count variable now holds the value returned by the loop
    //there is ";"  because the loop is being assigned as an expression, and the whole statement ends with a semicolon—just like any other variable assignment in Rust.

    println!("Final count from loop: {}", final_count);

    // Looping with a loop statement
    println!("****Looping with a loop statement!****");
    let mut counter = 10;
    loop {
        if counter >= 5 {
            break; // Exit the loop when counter is 5 or more
        }
        println!("Counter is: {}", counter);
        counter += 1;
    }
    let mut counting = 0;

    // Looping with a while loop
    while counting < 5 {
        println!("Counting: {}", counting);
        counting += 1;
    }

    // Looping through an array with a for loop
    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("Element: {}", element);
    }
    for number in 1..4 {
        // Range from 1 to 3 (exclusive of 4)
        println!("(Range from 1 to 3 (exclusive of 4) Number: {}", number);
    }
}
