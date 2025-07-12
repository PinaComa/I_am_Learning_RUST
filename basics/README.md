# 🦀 Learning Rust: Fundamentals in Practice

This repository contains hands-on Rust examples following Bogdan’s tutorial series. It covers key programming fundamentals with clear inline explanations and structured demonstrations.

---
# 🧠 Rust Basics Module

This module showcases foundational Rust programming concepts while following Bogdan's video tutorials. It serves as a hands-on guide to understanding variables, types, control flow, and loops through simple, functional examples.

## 🧭 Table of Contents

- [Variable Management](#variable-management)
- [Data Types](#data-types)
- [Functions](#functions)
- [Control Flow](#control-flow)
- [Loops](#loops)
- [Notable Examples](#notable-examples)

---

## 📝 Variable Management

- Mutable variables using `let mut`
- Reassigning and updating variable values
- Shadowing variables within nested scopes

## 🔢 Data Types

- **Integers** (`i32`, `u8`)
- **Floating-point numbers**
- **Booleans**
- **Characters**
- **Tuples** with destructuring and direct access
- **Arrays** with indexing, `.len()` and `Debug` printing

## 🧮 Functions

- `sum_multiply(x, y)`: Prints sum, returns product
- `conditionals()`: Explores `if`, `else if`, `else`, and match statements
- `loops()`: Demonstrates advanced loop control

## 🚦 Control Flow

- Conditional logic with `if` expressions
- Pattern matching using `match`

## 🔁 Loops

- `loop`: infinite loop with conditional `break` and value return
- `while`: condition-based loop
- `for`: iteration over arrays and ranges

## 💡 Notable Examples

```rust
let mut x = 5;
x = 6;

let x = 10; // shadowing outer x

let tup = ("lets ... ", 100_000);
let (text, number) = tup;
println!("Text: {}, Number: {}", text, number);

let arr = [1, 2, 3, 4, 5];
for element in arr.iter() {
    println!("Element: {}", element);
}

match number {
    1 => println!("One"),
    _ => println!("Something else"),
}
```
