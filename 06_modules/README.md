# 🔐 Auth Service (Rust Playground Project)

This is a lightweight authentication module written in Rust for educational and practice purposes. It models basic auth logic using modular design principles and idiomatic Rust patterns. 

---

## 📁 Module Structure

```text
crate auth_service
├── mod auth_utils: pub(crate)
│   ├── fn login: pub
│   ├── fn logout: pub(self)
│   └── mod models: pub
│       └── struct Credentials: pub
├── fn authenticate: pub
└── mod database: pub(crate)
    ├── enum Status: pub
    ├── fn connect_to_database: pub
    └── fn get_user: pub
```

---

## 🧩 Module Overview

### `lib.rs`
- Entry point for exposing public functionality.
- Re-exports `Credentials` for ergonomic access.
- Implements `authenticate()` which checks DB status and delegates login.

### `auth_utils.rs`
- Houses login/logout logic.
- Organizes credential handling via `models` submodule.
- Authenticates users by invoking `database::get_user()`.

### `auth_utils/models.rs`
Defines the Credentials struct:
```rust
pub struct Credentials {
    username: String,
    password: String,
}
```

### `database.rs`
Simulates database connectivity.
Contains:
`Status` enum: `Connected`, `Interrupted`
`connect_to_database() `→ returns a mock connection status
`get_user()` → placeholder for user retrieval logic

---

## ▶️ Sample Usage
```rust
use auth_service::authenticate;
use auth_service::Credentials;

fn main() {
    let creds = Credentials {
        username: String::from("pinar"),
        password: String::from("secret"),
    };
    authenticate(creds);
}
```

Expected Output:
```
Authenticated user: pinar
Fetching user data...
```

---

## 🔧 Development
```bash
cargo build
cargo run
cargo-modules structure
```
Add #`![allow(dead_code, unused_variables)]` to `lib.rs` to silence warnings during prototyping.


