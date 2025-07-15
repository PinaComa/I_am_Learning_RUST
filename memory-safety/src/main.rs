use unicode_segmentation::UnicodeSegmentation; // For grapheme segmentation
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

    // --- FIND SUBSTRING POSITION ---
    let text = String::from("Today is a very warm and sunny day.");
    let words = ["very", "arm", "say", "sun", "dew"];
    let mut pos;

    println!("Text: {text}");
    for word in words {
        pos = find_substr_pos(&text, word);
        if pos == text.len() {
            println!("{word} is not present in text");
        } else {
            println!("{word} present at index {pos}");
        }
    }

    // --- FIND SUBARRAY WITH GIVEN SUM ---
    println!("Finding subarray with sum 18 in the array [1, 1, 2, 3, 5, 8, 13]");
    // This function searches an array to find a subarray with the given sum
    // It returns the index where the subarray starts along with the length of the subarray
    let nums = [1, 1, 2, 3, 5, 8, 13];
    let res = find_subarray(&nums[..], 18);
    // If the array does not include any subarray with the sum, it returns a tuple with length of array
    if res.0 == nums.len() {
        println!("No subarray found");
    } else {
        println!("Subarray found: {:?}", &nums[res.0..res.0 + res.1]); // Print the found subarray
        println!(
            "Subarray starts at index {} and has length {}",
            res.0, res.1
        );
    }
    string_types(); // Call the function to demonstrate different string types in Rust
    manipulate_string(); // Call the function to demonstrate string manipulation in Rust
    concatenate_strings(); // Call the function to demonstrate string concatenation in Rust
    string_slicing(); // Call the function to demonstrate string slicing in Rust

    // --- FUNCTION WITH STRING SLICES ---
    let x = "Hello World!";
    let y = String::from("Hello World!");
    println!("my function of x: {}", my_function(x));
    // Passing a string slice to the function
    println!("my function of y: {}", my_function(&y));
    // Passing a String to the function, which will be automatically converted to a string slice
    // This demonstrates that you can pass both string slices and owned strings to functions that expect a string slice
    // The function my_function takes a string slice as an argument and returns a formatted string
    println!("my function of y: {}", my_function(&y[..]));
    // Passing a slice of the String to the function, which is also valid
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

fn find_substr_pos(text: &str, substr: &str) -> usize {
    // this function tries to search for substr in text from left to right
    // if it finds substr, it returns the index where it starts
    // otherwise it returns length of text (which is an invalid index)
    if text.len() < substr.len() {
        return text.len(); // If the text is shorter than the substring, return the length of the text
    }
    let len = substr.len();
    for start in 0..text.len() - len + 1 {
        // Iterate through the text to find the substring
        if substr == &text[start..start + len] {
            return start;
        }
    }
    text.len()
}

fn find_subarray(nums: &[i32], sum: i32) -> (usize, usize) {
    // this function searches an array to find a subarray with the given sum
    // it returns the index where the subarray starts along with the length of the subarray
    // if the array does not include any subarray with the sum, it returns a tuple with length or array
    for len in (1..nums.len() + 1).rev() {
        for start in 0..nums.len() - len + 1 {
            if array_sum(&nums[start..start + len]) == sum {
                return (start, len);
            }
        }
    }
    (nums.len(), nums.len())
}
fn array_sum(nums: &[i32]) -> i32 {
    // this function calculates the sum of an array and returns the sum of all elements
    let mut res = 0;
    for num in nums {
        res += num;
    }
    res
}

fn string_types() {
    // This function demonstrates different string types in Rust
    let str_literal: &str = "This is a string literal"; // String slice
    let str_owned: String = String::from("This is an owned string"); // Owned string
    let str_empty: String = String::new(); // Empty owned string
    let str_to_string: String = "to_string".to_string(); // Owned string using to_string()
    let str_to_owned: String = "to_owned".to_owned(); // Owned string using to_owned()
    let str_slice: &str = &str_to_owned[..]; // String slice from owned string

    // Print the different string types
    println!("String literal: {}", str_literal);
    println!("Owned string: {}", str_owned);
    println!("Empty owned string: {}", str_empty);
    println!("Owned string using to_string(): {}", str_to_string);
    println!("Owned string using to_owned(): {}", str_to_owned);
    println!("String slice from owned string: {}", str_slice);
    // Demonstrating that string slices can be used to refer to parts of owned strings
    println!("String slice from owned string: {}", &str_owned[0..4]); // "This" is a substring of str_owned
                                                                      // Demonstrating that string slices can be used to refer to parts of owned strings
                                                                      // This is useful for avoiding unnecessary copies and allowing multiple parts of your code to access the same data
                                                                      // Mutable references allow you to modify the data, but only one mutable reference can exist at a time to prevent
}

fn manipulate_string() {
    // This function demonstrates string manipulation in Rust
    let mut str_mut = String::from("Hello");
    str_mut.push_str(", world!"); // Append a string slice to the owned string
    println!("Modified string: {}", str_mut); // Print the modified string
    str_mut.push('!'); // Append a character to the owned string
    println!("After appending character: {}", str_mut); // Print the string after appending

    str_mut.replace_range(0..5, "Hi"); // Replace a range of characters in the string
    println!("After replacing range: {}", str_mut); // Print the string after replacing a range

    str_mut.clear(); // Clear the string, making it empty
    println!("After clearing: {}", str_mut); // Print the empty string
}

fn concatenate_strings() {
    // This function demonstrates string concatenation in Rust
    let str1 = String::from("Hello");
    let str2 = String::from("world");
    let str_format = format!("{} {} {}", str1, str2, "from format! macro"); // Concatenate two strings using format!
                                                                            // Concatenate two strings using format!
                                                                            // format! macro creates a new string without moving ownership of str1 or str2, copying the content of strings. LESS EFFICIENT THAN + OPERATOR!

    println!("Concatenated string: {}", str_format); // Print the concatenated string
    let str_add = str1 + " " + &str2; // Concatenate using the + operator
    println!("Concatenated string using + operator: {}", str_add);
    // Print the concatenated string using the + operator
    // Note: str1 is no longer valid after this
    // because it has been moved into the concatenation operation.
    // str2 is still valid, as it was not moved but borrowed
    println!("str2 is still valid: {}", str2);
    // Print str2 to show it is still valid
    // This demonstrates that the + operator moves ownership of the first string into the new string, while the second string is borrowed, allowing it to remain valid after the operation.
    let str_concat = concat!("foo", "bar"); // This macro concatenates literals at compile time, but it does not work with owned strings or string slices
    println!("Concatenated string using concat!: {}", str_concat); // Print
    let str_dot_concat = ["concating with ", "dot concat :)"].concat(); // Concatenate an array of string slices into a single owned string
    println!("Concatenated string using array concat: {}", str_dot_concat);
    // Print the concatenated string from the array
    // The format! macro gives String, so it can be used to concatenate owned strings and string slices
    // The concat! macro is used for string literals and gives a string slice &str, not an owned string
}

fn string_slicing() {
    // This function demonstrates string slicing in Rust
    let str_slice = "Hello, world!";
    let slice1 = &str_slice[0..5]; // Slice from index 0 to 5 (not inclusive of 5)
    let slice2 = &str_slice[7..12]; // Slice from index 7 to 12 (not inclusive of 12)
    println!("Slice 1: {}", slice1); // Print the first slice
    println!("Slice 2: {}", slice2); // Print the second slice
    let s_crab = "🦀🦀🦀🦀🦀";
    let s_crab_slices: &str = &s_crab[0..4];
    println!("{}", s_crab_slices);
    // Print the crab slices
    // Slicing works with Unicode characters, but you must ensure that the indices are valid UTF-8 character boundaries
    // If you slice at an invalid boundary, it will cause a runtime error
    // For example, slicing at a byte boundary that does not correspond to a valid character
    // let invalid_slice = &str_slice[0..6]; // This would cause a runtime error if the slice does not align with valid UTF-8 character boundaries
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // Iterate over the bytes of a string containing non-ASCII characters
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // [न म स त े] is a string containing non-ASCII characters
    // Iterate over the characters of a string containing non-ASCII characters
    // Rust's string slicing is powerful and allows you to work with both ASCII and non-ASCII characters seamlessly.
    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }
    // [न म स त े] is a string containing non-ASCII characters
    // Iterate over the graphemes of a string containing non-ASCII characters
    // Graphemes are the smallest units of a written language that represent a single character, which can be composed of multiple Unicode code points.
}
fn my_function(a: &str) -> String {
    format!(
        "{} - {}",
        a, "This is a string slice passed to the function"
    )
    // This function takes a string slice as an argument and returns a formatted string
}
