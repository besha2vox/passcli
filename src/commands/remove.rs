use crate::storage::{load_vault, save_vault};

pub fn handle_remove(service: &str) {
    let mut vault = load_vault();

    if vault.entries.remove(service).is_some() {
        save_vault(&vault, None);
        println!("✅ Password for '{}' has been removed.", service);
    } else {
        println!("❌ Service '{}' not found.", service);
    }
}
