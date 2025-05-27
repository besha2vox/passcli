use crate::{models::Vault, storage::load_vault};

pub fn choose_service_from_list(vault: Option<&Vault>) -> Option<String> {
    let vault_clone = match vault {
        Some(v) => v,
        None => &load_vault(),
    };

    if vault_clone.entries.is_empty() {
        println!("📭 No passwords saved yet.");
        return None;
    }

    let services: Vec<String> = vault_clone.entries.keys().cloned().collect();
    let service_index = dialoguer::Select::new()
        .with_prompt("Select a service")
        .items(&services)
        .default(0)
        .interact()
        .unwrap();

    let selected_service = services[service_index].clone();
    Some(selected_service)
}
