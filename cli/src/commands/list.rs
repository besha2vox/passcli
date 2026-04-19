use passcli_core::storage::load_vault;

pub fn handle_list() {
    let vault = load_vault();

    if vault.entries.is_empty() {
        println!("📭 No passwords saved yet.");
        return;
    }

    println!("🔐 Saved services:");
    let mut services: Vec<&String> = vault.entries.keys().collect();
    services.sort();

    for service in services {
        let accounts = &vault.entries[service];
        if accounts.len() == 1 {
            println!("  {} — {}", service, accounts[0].username);
        } else {
            println!("  {}", service);
            for account in accounts {
                println!("    • {}", account.username);
            }
        }
    }
}
