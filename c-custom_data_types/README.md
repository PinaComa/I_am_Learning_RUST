# 🦀 Rust Struct

how to define and use structs, methods, and associated functions in Rust—perfect 

---

## 📚 Overview

This mini-project introduces:
- Struct creation and usage
- Implementation blocks with methods and associated functions
- Utility functions for calculations
- Instantiation and printing with the `Debug` trait

The project revolves around two key structures: `User` and `Rectangle`.

---

## 🧱 Struct Definitions

### 👤 `User`

Represents a user with typical account fields.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
### 💡 Created with the utility function:
```rust
fn build_user(email: String, username: String) -> User
```

--

### 🟫 Rectangle
Used for geometry-related calculations and comparisons.
```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```
--

## 🛠 Methods vs Associated Functions in Rust

Methods and associated functions are both defined within `impl` blocks, but serve different purposes:

### 🔹 Methods
Defined inside an impl block and operate on an instance of the struct. They always take self, which can be:

#### 🧰 `self` Variants in Rust Methods

| `self` Variant | Description         | Ownership |
|----------------|---------------------|-----------|
| `&self`        | Immutable borrow     | ❌        |
| `&mut self`    | Mutable borrow       | ❌        |
| `self`         | Takes ownership      | ✅        |


Methods operate on instances and require `self` as a parameter.

#### 🔹 Examples

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

```

--

### 🔧 Associated Functions
Also in impl, but do not take self. Called using the type name, typically used for constructors or utility functions.
 
 #### 🔹 Example
```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```
✅ Called as: Rectangle::square(30)
🔄 Think of them like static functions in other languages.

--

### 🔨 Utility Functions
These calculate area using different data styles:
```rust
fn area_basic(width: u32, height: u32) -> u32
fn area_with_tuple(dimensions: (u32, u32)) -> u32
fn area_with_struct(rectangle: &Rectangle) -> u32
```
--

### ✨ Sample Output
```text
User1: pili, abc@gmail.com, 1, true
User2: qwe, qwe@gmail.com
User3: asd, asd@gmail.com
Area using basic function: 1500
Area using tuple: 1500
Area using struct: 1500
Does rect_struct hold rect_struct2? true
Does rect_struct hold rect_struct3? false
```

--

### 🧠 Concepts Covered
Struct design and update syntax
Methods vs associated functions
Ownership and borrowing
Automatic referencing in method calls
Pretty-printing with the Debug trait (#[derive(Debug)])

 !!Ctrl + Shift + V to preview in VS