use crate::storage::{decrypt_vault, load_vault, read_vault_raw, save_vault};
use crate::utils::is_vault_encrypted::*;
use rpassword::read_password;

pub fn handle_encrypt() {
    if is_vault_encrypted() {
        println!("Vault is already encrypted.");
        return;
    }

    println!("Enter encryption password:");
    let password = read_password().expect("Failed to read password");
    println!("Confirm encryption password:");
    let password_confirm = read_password().expect("Failed to read password");

    if password != password_confirm {
        eprintln!("Passwords do not match!");
        std::process::exit(1);
    }

    let mut vault = load_vault();

    if vault.encrypted {
        println!("Vault is already encrypted.");
        return;
    }

    vault.encrypted = true;

    save_vault(&vault, Some(&password));

    println!("Vault encrypted successfully.");
}

pub fn handle_decrypt() {
    if !is_vault_encrypted() {
        println!("Vault is not encrypted.");
        return;
    }

    println!("Enter decryption password:");
    let password = read_password().expect("Failed to read password");

    let encrypted_data = read_vault_raw();

    let mut vault = match decrypt_vault(&password, &encrypted_data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to decrypt vault: {}", e);
            std::process::exit(1);
        }
    };

    if !vault.encrypted {
        println!("Vault is already decrypted.");
        return;
    }

    vault.encrypted = false;

    save_vault(&vault, None);

    println!("Vault decrypted successfully.");
}
