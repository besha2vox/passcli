use crate::{
    models::{Account, Vault},
    storage::{load_vault, save_vault},
};

pub fn save_password(service: String, username: String, password: String, vault: Option<Vault>) {
    let mut vault = match vault {
        Some(v) => v,
        None => load_vault(),
    };
    vault
        .entries
        .entry(service)
        .or_insert_with(Vec::new)
        .push(Account { username, password });
    save_vault(&vault, None);
}
