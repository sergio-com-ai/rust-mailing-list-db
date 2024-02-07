mod email;

use email::Email;
use sled::Db;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

fn main() {
    // Open the Sled database
    let db: Db = sled::open("email_db").expect("Failed to open database");

    // Example operations
    let email1 = Email::new("example1@example.com".to_string());
    add_email(&db, &email1);
    let email2 = Email::new("example2@example.com".to_string());
    add_email(&db, &email2);

    // Retrieve and display an email by ID
    if let Some(email) = get_email(&db, &email1.id) {
        println!("Retrieved email: {}", email.email); // Modified to print just the email address
    }

    // List all emails
    println!("All emails:");
    list_emails(&db);
}

fn add_email(db: &Db, email: &Email) {
    let key = email.id.to_string();
    let value = serde_json::to_vec(email).expect("Failed to serialize email");
    db.insert(key.as_bytes(), value).expect("Failed to add email to DB");
    println!("Added email: {}", email.email);
}

fn get_email(db: &Db, id: &Uuid) -> Option<Email> {
    let key = id.to_string();
    if let Ok(Some(ivec)) = db.get(key.as_bytes()) {
        serde_json::from_slice(&ivec).ok()
    } else {
        None
    }
}

fn list_emails(db: &Db) {
    for result in db.iter() {
        let (_key, value) = result.expect("Error reading data");
        if let Ok(email) = serde_json::from_slice::<Email>(&value) {
            // Modified to print just the email address instead of the whole struct
            println!("{}", email.email);
        }
    }
}

