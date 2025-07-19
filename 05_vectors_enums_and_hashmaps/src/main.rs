use std::collections::HashMap; // Importing the HashMap from the standard library
                               // HashMap is a collection that allows you to store values associated with keys, similar to dictionaries in Python or objects in JavaScript.

fn main() {
    let a = [1, 2, 3]; // Using an array
    println!("Array: {:?}", a);
    let mut v: Vec<i32> = Vec::new(); // Using a vector
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    println!("Vector: {:?}", v);

    let first = v[0]; // Accessing the first element of the vector
    let second = &v[1]; // Borrowing the second element of the vector
    println!("First: {}, Second: {}", first, second);
    // the difference between accessing an element and borrowing it is that borrowing does not take ownership
    // and allows you to use the vector later without moving its elements - integer allows copying
    // while String does not allow copying, so you would need to clone it if you wanted to borrow it - in that case it would be &String or it would throw an error! it cannot access the second element of the vector because the first element is a string which cannot implement the Copy trait, so the vector is moved! but for integers it is fine.

    println!("Borrowed Second: {}", second);
    println!("First element: {}", first);
    println!("Vector: {}", v[1]);

    // let invalid = &v[100];
    // // This will panic at runtime if the index is out of bounds
    // println!("Invalid: {}", invalid); // Uncommenting this line will cause a panic
    let third = v[2]; // Accessing the third element of the vector
    println!("Third element: {}", third);

    //safer method is get method:
    match v.get(20) {
        Some(third) => println!("The selected element: {}", third),
        None => println!("No selected element"),
    }

    {
        let v2 = vec![1, 2, 3]; // Using the vec! macro to create a vector
        println!("Vector v2 inner scope: {:?}", v2);
    } // v2 goes out of scope here

    let mut vector = vec![0, 1, 2, 3, 4, 5];
    let third_element = &vector[2]; // Borrowing the third element
    println!("Third element: {}", third_element);
    vector.push(6); // Modifying the vector by adding a new element

    // println!("Third element: {}", third_element);
    // throws error cunku: şunlar birlikte çalışmaz:
    // let third_element = &vector[2]; → vector'dan immutably borrow ediyorsun.
    // vector.push(6); → vector'ı mutable olarak değiştirmeye çalışıyorsun.
    // Rust bu durumu potansiyel tehlike olarak görüyor. Çünkü push işlemi, vector'ın iç belleğini değiştirebilir (örneğin kapasite yetmezse yeniden allocate edilir). Bu durumda daha önce alınan third_element referansı geçersiz hale gelebilir. Rust, böyle belirsizliklere asla izin vermez.

    //as a reminder : let mut vector = vec![0, 1, 2, 3, 4, 5];
    // for i in vector {
    //     println!("{}", i);
    // }
    // This would move the vector, and you cannot use it after this point.
    for num in vector.iter() {
        println!("{}", num);
    } // Iterating over the vector using an immutable borrow

    for i in &vector {
        println!("{}", i);
    } // Iterating over the vector using a for loop
    for i in &mut vector {
        *i += 10; // Incrementing each element by 1
        println!("{}", i);
    }
    for i in &vector {
        println!("{}", i);
    } // iterating over the vector again to show the updated values

    // the difference between for i in vector and for i in &vector is that the first one takes ownership of the vector and moves it, while the second one borrows the vector immutably, allowing you to read its elements without taking ownership.
    // the difference between for i in &mut vector and for i in vector is that the second one takes ownership of the vector and moves it, while the first one borrows the vector mutably, allowing you to modify its elements without taking ownership. However, you cannot use both mutable and
    // immutable borrows at the same time, so you cannot use for i in &vector and for i in &mut vector at the same time, as it would cause a conflict.

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("Hello")),
    ];

    match &row[0] {
        SpreadSheetCell::Int(i) => println!("Integer: {}", i),
        _ => println!("Not an integer"),
    }
    // Using an enum to represent different types of data in a spreadsheet cell
    // This allows us to store different types of data in a single collection

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new(); // Creating a new HashMap
    scores.insert(blue, 10); // Inserting a key-value pair into the HashMap
    scores.insert(yellow, 50);
    // Inserting another key-value pair
    //not borrowing the key, but taking ownership of it
    // HashMap takes ownership of the key, so you cannot use the key after inserting it
    // If you want to borrow the key, you can use a reference like this:
    // scores.insert(&blue, 10); // This would borrow the key instead of taking ownership
    // but then you would need to use a reference to the key when accessing it,
    // like this: scores.get(&blue);

    println!("Scores: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // Accessing the value associated with the key

    match score {
        Some(s) => println!("Score for {}: {}", team_name, s),
        None => println!("No score found for {}", team_name),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value); // Iterating over the HashMap
    }

    //updating a value in the HashMap:
    scores.insert(String::from("Blue"), 10);
    // This will insert the key "Blue" with the value 10 if it does not exist
    scores.insert(String::from("Blue"), 20);
    // This will update the value associated with the key "Blue" to 20

    scores.entry(String::from("Yellow")).or_insert(30);
    // This will insert the key "Yellow" with the value 30 if it does not exist, but it already exists, it will not update the value!

    println!("Scores after entry API: {:?}", scores);

    //----//

    let text = "Hello world! wonderful world!";

    let mut map = HashMap::new();
    // Using split_whitespace to get an iterator over the words in the text:/
    for word in text.split_whitespace() {
        // Splitting the text into words

        let count = map.entry(word).or_insert(0); // Using entry API to insert
        *count += 1; // Incrementing the count for each word
    }
    println!("Word counts: {:?}", map);
}
