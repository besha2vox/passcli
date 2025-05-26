use crate::{
    models::PasswordEntry,
    storage::{load_vault, save_vault},
};
use std::io::{self, Write};

pub fn handle_update(service: &str) {
    let mut vault = load_vault();

    if !vault.entries.contains_key(service) {
        println!("Service \"{}\" not found.", service);
        return;
    }

    print!("Enter new password: ");
    io::stdout().flush().unwrap();

    let mut new_password = String::new();
    io::stdin().read_line(&mut new_password).unwrap();
    let new_password = new_password.trim().to_string();

    if new_password.is_empty() {
        println!("Password not updated. Input was empty.");
        return;
    }

    vault.entries.insert(
        service.to_string(),
        PasswordEntry {
            password: new_password,
            encrypted: vault.encrypted,
        },
    );
    save_vault(&vault, None);

    println!("Password for \"{}\" updated successfully.", service);
}
