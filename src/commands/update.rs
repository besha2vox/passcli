use crate::utils::choose_service_from_list::*;
use crate::{
    models::PasswordEntry,
    storage::{load_vault, save_vault},
};
use std::io::{self, Write};

pub fn handle_update() {
    let mut vault = load_vault();

    let service = choose_service_from_list(Some(&vault));
    if service.is_none() || service.as_deref().is_none() || service.as_deref().unwrap().is_empty() {
        println!("No service selected. Update cancelled.");
        return;
    }

    let service = service.unwrap();

    println!("🔐 Updating password for \"{}\".", service);
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
        service.clone(),
        PasswordEntry {
            password: new_password,
            encrypted: vault.encrypted,
        },
    );
    save_vault(&vault, None);

    println!("Password for \"{}\" updated successfully.", service.clone());
}
