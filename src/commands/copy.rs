use crate::storage::load_vault;
use arboard::Clipboard;

pub fn handle_copy(service: &str) {
    let vault = load_vault();

    if let Some(entry) = vault.entries.get(service) {
        let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
        clipboard
            .set_text(entry.password.clone())
            .expect("Error while copying");
        println!("📋 Password for '{}' copied to clipboard!", service);
    } else {
        println!("❌ Service '{}' not found.", service);
    }
}
