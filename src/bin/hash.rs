use bcrypt::{verify};

fn main() {
    let password = "admin123";
    let password_hash = "$2b$12$lsB.1mQxvVlND7dkAQh8vORIMWZxry1gttHtYU4rYGf1fSif2.uRO"; // hash dari DB

    match verify(password, password_hash) {
        Ok(valid) => println!("Password valid? {}", valid),
        Err(e) => println!("Error verifying password: {}", e),
    }
}

