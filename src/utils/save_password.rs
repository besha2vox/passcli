use crate::{
    models::{PasswordEntry, Vault},
    storage::{load_vault, save_vault},
};

pub fn save_password(service: String, password: String, vault: Option<Vault>) {
    let mut vault_clone = match vault {
        Some(v) => v,
        None => load_vault(),
    };
    vault_clone.entries.insert(
        service.clone(),
        PasswordEntry {
            password,
            encrypted: vault_clone.encrypted,
        },
    );
    save_vault(&vault_clone, None);
}
