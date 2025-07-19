#![allow(dead_code, unused_variables)]

mod database; // This module handles database connections and user data retrieval. The database module is defined in a separate file, so we can use it here.

mod auth_utils; // This module handles authentication utilities, including login and logout functions and the Credentials model. The auth_utils module is defined in a separate file, so we can use it here.

pub use auth_utils::models::Credentials; // Re-exporting the Credentials struct for easier access in other modules.
use database::Status;
pub fn authenticate(creds: Credentials) {
    if let status::Connected = database::connect_to_database() {
        println!("Authenticated user: {}", creds.username);
        auth_utils::login(creds);
    }
}
