use crate::storage::load_vault;
use crate::utils::copy_to_clipboard::*;
use crate::utils::generate_password::*;
use crate::utils::save_password::*;

use dialoguer::{Input, Password};

pub fn handle_add(generate: bool) {
    if generate {
        generate_and_add();
    } else {
        manual_add();
    }
}

fn manual_add() {
    let service: String = Input::new()
        .with_prompt("Enter service name")
        .interact_text()
        .unwrap();

    has_service(service.clone());

    let password: String = Password::new()
        .with_prompt("Enter password")
        .interact()
        .unwrap();

    save_password(service.clone(), password, None);
    println!("✅ Password for '{}' saved.", service);
}

fn generate_and_add() {
    let service: String = Input::new()
        .with_prompt("Enter service name")
        .interact_text()
        .unwrap();

    has_service(service.clone());

    let password_options = crate::utils::generate_password::choose_pass_options();
    let generated = generate_password(password_options.1, password_options.0, password_options.2);
    save_password(service.clone(), generated.clone(), None);
    copy_to_clipboard(&generated);
    println!(
        "✅ Password for '{}' saved and copied to clipboard.",
        service
    );
}

pub fn has_service(service: String) -> () {
    let vault = load_vault();

    let is_exists = vault.entries.get(service.as_str()).is_some();

    if is_exists {
        println!(
            "❌ Service '{}' already exists. Use 'update' to change it.",
            service
        );
        std::process::exit(1);
    }
}
