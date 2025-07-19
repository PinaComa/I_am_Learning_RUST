# 🦀 Rust Collections 

how Rust handles arrays, vectors, enums, and hash maps—all essential tools for efficient data management.

## 📌 What’s Inside?

The `main.rs` demonstrates:
- Basic array and vector operations
- Safe element access techniques
- Iterating and modifying collections
- Using enums for mixed-type storage
- HashMap operations: insertion, retrieval, updates
- Counting word occurrences using a hash map

## 🧠 Core Concepts Illustrated

### ✅ Arrays and Vectors
```rust
let a = [1, 2, 3];
let mut v: Vec<i32> = Vec::new();
```
Explore creating collections, adding elements, and accessing them by index.

---


### 🛡️ Safe Access
```rust
match v.get(20) {
    Some(third) => println!("Element: {}", third),
    None => println!("No selected element"),
}
```
Prevent runtime panics with .get() for secure indexing.

---

### 🔄 Iteration and Mutation
```rust
for i in &mut vector {
    *i += 10;
}
```
Demonstrates modifying vector elements and looping through collections efficiently.

---

### 📊 Enums as Flexible Data Types
```rust
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
```
Models a spreadsheet-style row where each cell can store different data types.

---

### 🗺️ HashMaps in Action
```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
```
Insert key-value pairs, retrieve scores, iterate through entries, and use .entry() for conditional updates.

---

### 📚 Word Counter
```rust
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```
Use a HashMap to count word frequency—great intro to text analysis!

