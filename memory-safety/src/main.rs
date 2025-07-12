fn main() {
    println!("****This is the copy or move function - or ownership !****");

    // --- COPY VS MOVE WITH STRINGS ---
    let str_orig = String::from("Rust");

    /* let str_moved = str_orig;
    println!("String str_orig: {str_orig}"); // This will cause an error because str_orig has been MOVED, not copied
    println!("String str_moved: {str_moved}"); // This is fine, as str_moved is a valid variable now */

    let mut str_clone = str_orig.clone(); // Cloning str_orig to str_clone, so both can be used independently
    str_clone.push_str(" is an awesome language"); // Modifying str_clone, which is a clone of str_orig

    println!("String str_orig: {str_orig}"); // str_orig is still valid because we cloned
    println!("String str_clone: {str_clone}"); // str_clone is now a modified version of str_orig
    println!("String:\"{str_orig}\" is a substring of \"{str_clone}\"");

    // --- COPY TRAIT WITH INTEGERS ---
    let num_x = 5;
    let num_y = num_x; // num_y is a COPY of num_x, as integers implement the Copy trait, NOT MOVE!!
                       /* [integers, booleans, and characters are Copy types] */
    println!("num_x: {num_x}, num_y: {num_y}"); // Both num_x and num_y are valid and independent

    // --- OWNERSHIP WITH STRINGS ---
    let str_take = String::from("Hello");
    takes_ownership(str_take); // str_take's ownership is moved to the function
                               // println!("str_take: {str_take}"); // This would cause an error because str_take is no longer valid

    // --- COPY AGAIN WITH AN INTEGER ---
    let num_z = 5;
    makes_copy(num_z); // num_z is copied, so it remains valid after the function call
    println!("num_z: {}", num_z); // This is fine, as num_z is still valid

    // --- RETURNING OWNERSHIP ---
    let str_got = gives_ownership(); // Ownership of the string is returned to str_got
    println!("str_got: {}", str_got); // str_got is now valid and holds the returned string

    // --- TAKING AND GIVING BACK OWNERSHIP ---
    let str_sent = String::from("Hello again");
    let str_back = takes_and_gives_back(str_sent); // Ownership moved to function, then returned
    println!("str_back: {}", str_back); // str_back holds the returned string
                                        // str_sent is no longer valid here, as its ownership was moved

    // --- CALCULATING LENGTH OF A STRING ---
    let str_len = String::from("hellooo");
    let (mut str_len, len_val) = calculate_length_bad(str_len); // Ownership moved in, returned out
    println!("The length of '{}' is {}", str_len, len_val); // ✅ works
    println!("str_len: {}", str_len);
    // str_len is still valid, as we returned it from the function
    // this is the new str_len; the old one is shadowed
    str_len.push_str(" - modified"); // We can modify str_len here
    println!("Modified str_len: {}", str_len);

    // --- CALCULATE LENGTH USING REFERENCES TO AVOID MOVING OWNERSHIP ---
    // BORROWING

    let str_ref = String::from("hellooo");

    let len_ref = calculate_length(&str_ref);
    // Here we pass a REFERENCE to str_ref, so we don't move ownership
    // str_ref is still valid after this function call
    println!("The length of '{}' is {}", str_ref, len_ref);
    println!("str_ref: {}", str_ref); // str_ref is still valid and can be used

    // --- MUTABLE REFERENCES ---
    let mut str_mut = String::from("hello");
    change(&mut str_mut); // We pass a mutable reference to str_mut
    println!("Modified str_mut: {}", str_mut);

    // --- DANGLE EXAMPLE ---
    // let dangling_ref = dangle(); // This would cause a compile-time error because dangling_ref would point to a value that goes out of scope
    // println!("Dangling reference: {}", dangling_ref); // dangling_ref is not valid here
    // Instead, we can return ownership directly:
    let owned_str = String::from("This is a valid string");
    println!("Owned string: {}", owned_str); // This is valid, as we own the string
                                             // --- FIRST WORD FUNCTION ---
    let str_first_word = String::from("Hello world");
    let hello = &str_first_word[0..5]; // Get a slice of the first word
    let world = &str_first_word[6..11]; // Get a slice of the second word

    let s_new = "hello world";
    let word = first_word(s_new); // Call the first_word function with a string slice
    println!("First word: {}", word); // Print the first word

    // [str_first_word.clear(); // Clear the original string
    //ERROR : mutable borrow occurs here ]
    println!("First word: {}, Second word: {}", hello, world);

    slice_on_arrays(); // Call the function to demonstrate slicing on arrays
}

// --- REFERENCES AND BORROWING ---
// References allow you to refer to some value without taking ownership of it.
// This is useful for avoiding unnecessary copies and allowing multiple parts of your code to access the same data.
// Mutable references allow you to modify the data, but only one mutable reference can exist at a time to prevent

// we cannot borrow a value mutable more than once at a time!
// prevent data races at compile time and ensure memory safety. a data race occurs when two or more threads access shared data and at least one thread modifies the data without proper synchronization.
// This is enforced by the Rust compiler, which checks for these conditions at compile time.

// multiple immutable references can exist at the same time, but you cannot have a mutable reference while immutable references exist.
// This ensures that you cannot have a mutable reference to data while it is also being accessed
// however after the immutable references go out of scope, you can create a mutable reference to the same data. BELOW IS AN EXAMPLE OF THIS:

/* let str_demo = String::from("hello");
let r1 = &str_demo;
let r2 = &str_demo;
println!("{},{}", r1,r2);

let r3 = &mut str_demo;
println!("{}", r3);
OK
 */

// --- SHADOWING ---
// Shadowing allows you to declare a new variable with the same name as a previous variable, effectively "hiding" the previous one.
// This can be useful for reusing names in different scopes or for transforming values without needing to create new variable names.
// Shadowing is different from mutability; it allows you to change the type of the variable or its value while keeping the same name.

// --- OWNERSHIP FUNCTION ---
fn takes_ownership(some_str: String) {
    println!("Taking ownership of: {}", some_str); // some_str goes out of scope here and is dropped
}

// --- COPY FUNCTION ---
fn makes_copy(some_num: i32) {
    println!("Making a copy of: {}", some_num);
}

fn gives_ownership() -> String {
    let str_new = String::from("This is an owned string");
    str_new // Ownership of the string is returned
}

fn takes_and_gives_back(str_input: String) -> String {
    println!("Taking and giving back: {}", str_input);
    str_input // Returns the string
}

fn calculate_length_bad(s: String) -> (String, usize) {
    let len = s.len(); // Calculate the length of the string
    (s, len)
}

fn calculate_length(s: &String) -> usize {
    let len = s.len(); // Calculate the length of the string
    len
}

fn change(s: &mut String) {
    s.push_str(" - modified"); // Modify the string
}

/*fn dangle() -> &String {
    let s = String::from("This will dangle");
    &s // ❌ compile-time error: s goes out of scope
    // return s; // ✅ valid, if returning ownership
}
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // Convert the string to bytes
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Check for space character
            return &s[0..i]; // Return the word until the index of the first space
        }
    }
    &s[..] // Return the entire string if no space is found
           // s.len() // If no space found, return the length of the string
}

fn slice_on_arrays() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[0..2]; // Create a slice from index 1 to 3 (not inclusive of 4)
    println!("Slice of the array: {:?}", slice); // Print the slice
}
