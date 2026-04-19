use passcli_core::storage::load_vault;
use passcli_core::utils::copy_to_clipboard::copy_to_clipboard;
use dialoguer::Select;

pub fn handle_copy(service: &str) {
    let vault = load_vault();

    match vault.entries.get(service) {
        None => println!("❌ Service '{}' not found.", service),
        Some(accounts) if accounts.is_empty() => {
            println!("❌ No accounts for '{}'.", service)
        }
        Some(accounts) if accounts.len() == 1 => {
            copy_to_clipboard(&accounts[0].password);
            println!(
                "📋 Password for '{}/{}' copied to clipboard!",
                service, accounts[0].username
            );
        }
        Some(accounts) => {
            let labels: Vec<&str> = accounts.iter().map(|a| a.username.as_str()).collect();
            let idx = Select::new()
                .with_prompt("Select account")
                .items(&labels)
                .default(0)
                .interact()
                .unwrap();
            copy_to_clipboard(&accounts[idx].password);
            println!(
                "📋 Password for '{}/{}' copied to clipboard!",
                service, accounts[idx].username
            );
        }
    }
}
