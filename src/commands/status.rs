use crate::storage::{get_vault_path, load_vault};

pub fn handle_status() {
    let path = get_vault_path();
    let vault = load_vault();

    println!("Vault path: {}", path.display());
    println!("Encrypted: {}", if vault.encrypted { "Yes" } else { "No" });
    println!("Entries: {}", vault.entries.len());
}
