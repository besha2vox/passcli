use std::process::exit;

use crate::models::PasswordEntry;
use crate::storage::{load_vault, save_vault};
use crate::utils::copy_to_clipboard::*;

use dialoguer::{Input, Password, Select};
use rand::random_range;

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

    save_password(service.clone(), password);
    println!("✅ Password for '{}' saved.", service);
}

fn generate_and_add() {
    let service: String = Input::new()
        .with_prompt("Enter service name")
        .interact_text()
        .unwrap();

    has_service(service.clone());

    let case_options = &["lower", "upper", "mixed"];
    let case_index = Select::new()
        .with_prompt("Select case")
        .items(case_options)
        .default(2)
        .interact()
        .unwrap();
    let selected_case = case_options[case_index];

    let length_input: String = Input::new()
        .with_prompt("Specify password length (e.g., 16 or 8-20)")
        .interact_text()
        .unwrap();

    let length = parse_length(&length_input);

    let use_specials: String = Input::new()
        .with_prompt("Include special characters? (y/n)")
        .interact_text()
        .unwrap();

    let use_specials = matches!(use_specials.to_lowercase().as_str(), "y");

    let generated = generate_password(length, selected_case, use_specials);

    save_password(service.clone(), generated.clone());
    copy_to_clipboard(&generated);
    println!(
        "✅ Password for '{}' saved and copied to clipboard.",
        service
    );
}

fn save_password(service: String, password: String) {
    let mut vault = load_vault();
    vault.entries.insert(
        service.clone(),
        PasswordEntry {
            password,
            encrypted: vault.encrypted,
        },
    );
    save_vault(&vault, None);
}

fn parse_length(input: &str) -> usize {
    if let Some((a, b)) = input.split_once('-') {
        let mut min = a.trim().parse::<usize>().unwrap_or(8);
        let mut max = b.trim().parse::<usize>().unwrap_or(16);
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }
        random_range(min..=max)
    } else {
        input.trim().parse::<usize>().unwrap_or(16)
    }
}

fn generate_password(length: usize, case: &str, use_specials: bool) -> String {
    let mut charset = match case {
        "lower" => "abcdefghijklmnopqrstuvwxyz".to_string(),
        "upper" => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        "mixed" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        _ => "abcdefghijklmnopqrstuvwxyz".to_string(),
    };

    charset.push_str("0123456789");

    if use_specials {
        charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>/?");
    }

    (0..length)
        .map(|_| {
            let idx = random_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
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
