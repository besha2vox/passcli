use crate::{models::Vault, storage::load_vault};

pub fn choose_service_from_list(vault: Option<&Vault>) -> Option<String> {
    let owned;
    let vault_ref = match vault {
        Some(v) => v,
        None => {
            owned = load_vault();
            &owned
        }
    };

    if vault_ref.entries.is_empty() {
        println!("📭 No passwords saved yet.");
        return None;
    }

    let services: Vec<String> = vault_ref.entries.keys().cloned().collect();
    let service_index = dialoguer::Select::new()
        .with_prompt("Select a service")
        .items(&services)
        .default(0)
        .interact()
        .unwrap();

    Some(services[service_index].clone())
}
