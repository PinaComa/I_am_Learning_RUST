# 🦀 Rust Cheat Sheet - BOGDAN

This cheat sheet covers core language features, idioms, and code patterns to accelerate your learning and development.

👉 For video tutorials and practical examples, check out: [Lets Get Rusty on YouTube](https://youtube.com/LetsGetRusty)

---

## 📚 Table of Contents

- [Basic Types & Variables](#-basic-types--variables)
- [Control Flow](#-control-flow)
- [Ownership, References & Borrowing](#-ownership-references--borrowing)
- [Pattern Matching](#-pattern-matching)
- [Iterators](#-iterators)
- [Error Handling](#-error-handling)
- [Combinators](#-combinators)
- [Multiple Error Types](#-multiple-error-types)
- [Iterating Over Errors](#-iterating-over-errors)
- [Generics, Traits & Lifetimes](#-generics-traits--lifetimes)
- [Functions, Function Pointers & Closures](#-functions-function-pointers--closures)
- [Pointers & Smart Pointers](#-pointers--smart-pointers)
- [Packages, Crates & Modules](#-packages-crates--modules)
- [Other Goodies](#-other-goodies)
- [Reference](#-reference)


---

## 🧩 Basic Types & Variables 

- Boolean: `bool`
- Integers: `u8` → `u128`, `i8` → `i128`, `usize`, `isize`
- Floating point: `f32`, `f64`
- Character: `char`
- Strings: `&str`, `String`
- Tuple: `(value1, value2)`
- Arrays: `[1, 2, 3]`, `[0; 3]`
- Slice: `&array[start..end]`

```rust
let score = ("Team A", 12);
let array = [1, 2, 3, 4, 5];
let slice = &array[1..3];
```

---

## 🔁 Control Flow
if, if let, match
Loops: loop, while, for, while let
Loop labels & breaking
Shadowing & mutability
```rust
let x = 5;
let x = x * 2; // Shadowing
```

---

## 🔐 Ownership, References & Borrowing
Ownership Rules
Each value has one owner.
Only one owner at a time.
Value is dropped when owner goes out of scope.
Borrowing Rules
One mutable reference or multiple immutable ones
References must be valid
```rust
let mut s = String::from("Hello");
let s_ref = &mut s;
s_ref.push_str(" World!");
```
---

## 🎯 Pattern Matching
match with literals, ranges, destructuring
Enum variants, struct fields
Ignoring with _, match guards, @ bindings
```rust
match x {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    _ => println!("something else"),
}
```
---

## 🔄 Iterators
.iter(), .map(), .collect(), .sum()
Creating custom iterators with impl Iterator
```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
```

---

## 🚨 Error Handling
panic!() for unrecoverable errors
Option<T> and Result<T, E>
? operator for propagation
Multiple error types and combinators
```rust
fn get_salary(db: Database, id: i32) -> Option<u32> {
    Some(db.get_user(id)?.get_job()?.salary)
}
```

---

## 🧬 Generics, Traits & Lifetimes
Generic types: fn<T>(value: T)
Trait bounds: T: Display
Lifetimes: &'a str

## ⚡️ Functions & Closures
Standard function syntax
Function pointers
Closures with inferred or explicit types
```rust
let add = |x: i32, y: i32| x + y;
```

---

## 📍 Pointers & Smart Pointers
Box<T>, Rc<T>, RefCell<T> for heap allocation, shared ownership, and interior mutability

## 📦 Packages, Crates & Modules
Organized with Cargo.toml
Use mod, pub, and use for code reuse

## 🌈 Other Goodies
Constants: const MAX_POINTS: u32 = 100_000;
Static variables: static VERSION: u32 = 1;

---

## 🔀 Combinators

Elegant transformations and chaining over `Option` and `Result` types using methods like `.map`, `.and_then`, and more.

### `.map`
```rust
let name = Some("LGR".to_owned());
let name_len = name.map(|s| s.len());

let result: Result<String, Error> = Ok("Bogdan".to_owned());
let user: Result<User, Error> = result.map(|name| User { name });
let vec = Some(vec![1, 2, 3]);
let first = vec.and_then(|v| v.into_iter().next());

let str_result: Result<&str, _> = Ok("5");
let num_result = str_result.and_then(|s| s.parse::<u32>());

```

---

## ⚠️ Multiple Error Types
### 💡 Custom Error Type
```rust
type Result<T> = std::result::Result<T, CustomError>;

#[derive(Debug, Clone)]
struct CustomError;

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "custom error message")
    }
}
```
---


## 📦 Boxed Errors
```rust
use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
```
---

## 🔁 Iterating Over Errors
✅ Filter valid entries
```rust
let strings = vec!["LGR", "22", "7"];
let numbers: Vec<_> = strings
    .into_iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();
```

#### ❌ Fail fast with collect()
```rust
let strings = vec!["LGR", "22", "7"];
let result: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
```

---

## 🔀 Partition values & errors
```rust
let strings = vec!["LGR", "22", "7"];
let (oks, errs): (Vec<_>, Vec<_>) = strings
    .into_iter()
    .map(|s| s.parse::<i32>())
    .partition(Result::is_ok);

let numbers: Vec<_> = oks.into_iter().map(Result::unwrap).collect();
let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();
```
---

## 🧬 Generics, Traits & Lifetimes
Generic structs and methods
Trait bounds and default implementations
Supertraits and trait objects
Lifetimes in functions and structs
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```
---

## 🔧 Functions, Function Pointers & Closures
Associated functions and methods
Closures: Fn, FnMut, FnOnce
Returning and storing closures
Function pointers and higher-order functions
```rust
fn do_twice<T>(f: T, x: i32) -> i32 where T: Fn(i32) -> i32 {
    f(x) + f(x)
}
```
---

## 🔗 Pointers & Smart Pointers

Rust provides various smart pointers for memory management and ownership control:

| Pointer Type | Purpose                                 |
|--------------|------------------------------------------|
| `Box<T>`     | Heap allocation                          |
| `Rc<T>`      | Reference counting (shared ownership)    |
| `RefCell<T>` | Interior mutability checked at runtime   |

---

## 🧱 Packages, Crates & Modules
Organizing Rust code into logical, reusable, and scoped units.
```bash
# Create binary crate
cargo new my_project

# Create library crate
cargo new my_lib --lib
```

Modules & Re-exports

```rust
mod outer {
    pub mod inner {
        pub fn greet() {
            println!("Hello from inner!");
        }
    }
}

pub use outer::inner; // Re-export
```


## 🎥 Reference
https://youtube.com/LetsGetRusty



