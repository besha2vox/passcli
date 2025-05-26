use crate::storage::load_vault;

pub fn handle_list() {
    let vault = load_vault();

    if vault.entries.is_empty() {
        println!("📭 No passwords saved yet.");
    } else {
        println!("🔐 Saved services:");
        for service in vault.entries.keys() {
            println!("- {}", service);
        }
    }
}
