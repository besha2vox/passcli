use crate::models::Vault;
use crate::storage::load_vault;
use crate::utils::choose_service_from_list::*;
use crate::utils::copy_to_clipboard::*;
use crate::utils::generate_password::*;
use crate::utils::save_password::*;
use std::io::{self, Write};

pub fn handle_update(generate: bool) {
    if generate {
        generate_and_update();
    } else {
        manual_update();
    }
}

fn manual_update() {
    let vault = load_vault();

    let service = get_service_from_user(&vault);

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

    save_password(service.clone(), new_password.clone(), Some(vault));
    println!("Password for \"{}\" updated successfully.", service);
}

fn generate_and_update() {
    let vault = load_vault();

    let service = get_service_from_user(&vault);

    let password_options = crate::utils::generate_password::choose_pass_options();
    let generated = generate_password(password_options.1, password_options.0, password_options.2);

    save_password(service.clone(), generated.clone(), Some(vault));
    copy_to_clipboard(&generated);
    println!(
        "Password for \"{}\" updated successfully with generated password and copied to clipboard.",
        service
    );
}

fn get_service_from_user(vault: &Vault) -> String {
    let service = choose_service_from_list(Some(&vault));
    if service.is_none() || service.as_deref().is_none() || service.as_deref().unwrap().is_empty() {
        println!("No service selected. Update cancelled.");
        std::process::exit(1);
    }

    let service = service.unwrap();
    service
}
