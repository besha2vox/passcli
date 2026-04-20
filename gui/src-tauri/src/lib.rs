use passcli_core::{models::Account, storage, utils};
use serde::Serialize;

#[derive(Serialize)]
pub struct VaultEntry {
    pub service: String,
    pub accounts: Vec<Account>,
}

#[tauri::command]
fn list_entries() -> Vec<VaultEntry> {
    let vault = storage::load_vault();
    let mut entries: Vec<VaultEntry> = vault
        .entries
        .into_iter()
        .map(|(service, accounts)| VaultEntry { service, accounts })
        .collect();
    entries.sort_by(|a, b| a.service.cmp(&b.service));
    entries
}

#[tauri::command]
fn add_account(service: String, username: String, password: String) -> Result<(), String> {
    let vault = storage::load_vault();
    if let Some(accounts) = vault.entries.get(&service) {
        if accounts.iter().any(|a| a.username == username) {
            return Err(format!(
                "Account '{}' for '{}' already exists",
                username, service
            ));
        }
    }
    utils::save_password::save_password(service, username, password, Some(vault));
    Ok(())
}

#[tauri::command]
fn update_account(service: String, username: String, new_password: String) -> Result<(), String> {
    let mut vault = storage::load_vault();
    let accounts = vault
        .entries
        .get_mut(&service)
        .ok_or_else(|| format!("Service '{}' not found", service))?;
    let account = accounts
        .iter_mut()
        .find(|a| a.username == username)
        .ok_or_else(|| format!("Account '{}' not found", username))?;
    account.password = new_password;
    storage::save_vault(&vault, None);
    Ok(())
}

#[tauri::command]
fn copy_password(service: String, username: String) -> Result<(), String> {
    let vault = storage::load_vault();
    let accounts = vault
        .entries
        .get(&service)
        .ok_or_else(|| format!("Service '{}' not found", service))?;
    let account = accounts
        .iter()
        .find(|a| a.username == username)
        .ok_or_else(|| format!("Account '{}' not found", username))?;
    utils::copy_to_clipboard::copy_to_clipboard(&account.password);
    Ok(())
}

#[tauri::command]
fn generate_password_cmd(length: usize, case: String, use_specials: bool) -> String {
    utils::generate_password::generate_password(length, &case, use_specials)
}

#[tauri::command]
fn remove_account(service: String, username: Option<String>) -> Result<(), String> {
    let mut vault = storage::load_vault();
    match username {
        Some(u) => {
            let accounts = vault
                .entries
                .get_mut(&service)
                .ok_or_else(|| format!("Service '{}' not found", service))?;
            let pos = accounts
                .iter()
                .position(|a| a.username == u)
                .ok_or_else(|| format!("Account '{}' not found", u))?;
            accounts.remove(pos);
            if accounts.is_empty() {
                vault.entries.remove(&service);
            }
        }
        None => {
            vault.entries.remove(&service);
        }
    }
    storage::save_vault(&vault, None);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_entries,
            add_account,
            update_account,
            copy_password,
            generate_password_cmd,
            remove_account,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
