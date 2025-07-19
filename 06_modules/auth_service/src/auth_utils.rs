pub fn login(creds: models::Credentials) {
    crate::database::get_user();
}

fn logout() {
    println!("User logged out successfully.");
}

pub mod models;
