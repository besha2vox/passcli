use passcli_core::storage::{load_vault, save_vault};
use dialoguer::Select;

pub fn handle_remove(service: &str) {
    let mut vault = load_vault();

    let accounts = match vault.entries.get(service) {
        None => {
            println!("❌ Service '{}' not found.", service);
            return;
        }
        Some(a) => a,
    };

    if accounts.len() == 1 {
        let username = accounts[0].username.clone();
        vault.entries.remove(service);
        save_vault(&vault, None);
        println!("✅ Account '{}' for '{}' removed.", username, service);
        return;
    }

    let mut labels: Vec<String> = accounts.iter().map(|a| a.username.clone()).collect();
    labels.push("⚠️  Remove entire service".to_string());

    let idx = Select::new()
        .with_prompt("Select account to remove")
        .items(&labels)
        .default(0)
        .interact()
        .unwrap();

    if idx == labels.len() - 1 {
        vault.entries.remove(service);
        save_vault(&vault, None);
        println!("✅ Service '{}' and all its accounts removed.", service);
    } else {
        let removed_username = labels[idx].clone();
        let accounts = vault.entries.get_mut(service).unwrap();
        accounts.remove(idx);
        if accounts.is_empty() {
            vault.entries.remove(service);
        }
        save_vault(&vault, None);
        println!("✅ Account '{}' removed from '{}'.", removed_username, service);
    }
}
