# 🧩 Rust Enum - Matching - Options

This project explores Rust's powerful `enum` capabilities, including custom enum types, pattern matching, and `Option<T>` for handling absence of values. It also includes methods on enums and demonstrates real-world use cases like modeling IP addresses and coin values.

---

## 📦 Key Enums

### 🧭 `IpAddrKind`

Represents IP address types with relevant associated data.

```rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
```
Used inside the IpAddr struct:

```rust
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
```
✅ Called with: let localhost = IpAddrKind::V4(127, 0, 0, 1);

---

## 💬 Message
Demonstrates different enum variants:
- No data
- Struct-like named fields
- Single values
- Tuple-style payloads

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
Includes a method:
```rust
impl Message {
    fn call(&self) {
        println!("Message called");
    }
}
```
📌 Called with: msg.call();

---

## 💰 Coin and UsState
Custom enum with nested variant containing another enum.
```rust
#[derive(Debug)]
enum UsState {
    Alabama, Alaska, Arizona, Arkansas, California, // …
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```
Used in:
```rust
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        },
        // others...
    }
}
```
---

## 🎯 Option Type
Rust’s built-in Option<T> is used for safe handling of missing values.
```rust
let some_number = Option::Some(5);
let absent_number: Option<i32> = Option::None;

let sum = a + b.unwrap_or(0); // Safely handling None
```
Pattern matching with Option:
```rust
match some_value {
    Some(10) => println!("Value is 10"),
    _ => (),
}

if let Some(10) = some_value {
    println!("Value is 10 using if let");
}
```

---

## 🔧 Utility Functions
```rust
fn route(ip_kind: IpAddrKind) { /* matches V4 and V6 */ }

fn value_in_cents(coin: Coin) -> u32 { /* matches all coin variants */ }

fn plus_one(x: Option<i32>) -> Option<i32> { /* matches Some or None */ }
```

## 🧠 What You'll Learn
- Defining and working with custom enums
- Attaching data to enum variants
- Pattern matching with match and if let
- Using methods and impl blocks on enums
- Safe handling of optional values with Option<T>