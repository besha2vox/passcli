use crate::storage::load_vault;
use crate::utils::copy_to_clipboard::*;

pub fn handle_copy(service: &str) {
    let vault = load_vault();

    if let Some(entry) = vault.entries.get(service) {
        copy_to_clipboard(&entry.password.clone());
        println!("📋 Password for '{}' copied to clipboard!", service);
    } else {
        println!("❌ Service '{}' not found.", service);
    }
}
