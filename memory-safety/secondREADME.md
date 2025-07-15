# 🦀 Memory Safety in Rust — Ownership, Strings & Unicode Mastery

---

## 📚 Table of Contents

1. [Project Overview](#-project-overview)
2. [Dependencies](#-dependencies)
3. [Ownership & Copy](#-ownership--copy)
4. [Borrowing & References](#-borrowing--references)
5. [Dangling References](#-dangling-references)
6. [String Length Calculations](#-string-length-calculations)
7. [String Slicing & First Word](#-string-slicing--first-word)
8. [Substrings & Subarrays](#-substrings--subarrays)
9. [String Types & Manipulation](#-string-types--manipulation)
10. [String Concatenation](#-string-concatenation)
11. [Unicode Exploration](#-unicode-exploration)
12. [Function Using String Slice](#-function-using-string-slice)

---

## 🚀 Project Overview

This project demonstrates how Rust ensures memory safety by enforcing rules around **ownership**, **borrowing**, and **string management**—all backed by compile-time guarantees.

---

## 📦 Dependencies

```toml
[dependencies]
unicode-segmentation = "1.10.0"
```

This crate enables advanced Unicode handling like grapheme segmentation, which helps with international string processing.

## 🔄 Ownership & Copy
takes_ownership(String)
makes_copy(i32)
gives_ownership()
takes_and_gives_back(String)

These functions cover the mechanics of ownership transfer vs implicit copying in Rust.

---

## 🧷 Borrowing & References
Immutable: calculate_length(&String)
Mutable: change(&mut String)
Slices: first_word(&str)

Rust ensures safe access to data through borrowing rules, allowing read or write—but never both at once!

---

## ❌ Dangling References
Commented-out dangle() is a perfect illustration of Rust preventing unsafe references to data that has gone out of scope.
 
---

## 📏 String Length Calculations
calculate_length_bad(String): transfers ownership
calculate_length(&String): borrows reference
Both achieve the same goal, but highlight different techniques.

---

## ✂️ String Slicing & First Word
Slice strings directly: &str[0..n]
Extract first word: first_word(&str)

Covers how Rust lets you safely segment strings—even across multilingual character sets.

---

## 🧩 Substrings & Subarrays
find_substr_pos(&str, &str)
find_subarray(&[i32], i32)
array_sum(&[i32])

Search and extract meaningful patterns using custom logic.

---

## 🔡 String Types & Manipulation
Demonstrates: String, &str, .to_string(), .to_owned()
Function: string_types(), manipulate_string()
Covers fundamental string creation, mutation, and formatting techniques.

---

## 🔗 String Concatenation
Format: format!()
Operator: +
Macros: concat!()
Array join: ["a", "b"].concat()
Function: concatenate_strings() demonstrates each approach with pros and cons.

---

## 🌐 Unicode Exploration
Function: string_slicing()
Includes:
Iterating over bytes: .bytes()
Characters: .chars()
Graphemes: .graphemes(true)
Features multilingual support including Hindi (नमस्ते) and emoji (🦀)—showing off Rust’s precise Unicode handling.

---

## 🧪 Function Using String Slice
my_function(&str) -> String
You can pass both string literals and owned strings, showing flexibility in Rust’s type system.

---

## ✅ How to Run
``` bash
cargo build
cargo run
```
Make sure the dependencies are set, and you're good to go!

---

