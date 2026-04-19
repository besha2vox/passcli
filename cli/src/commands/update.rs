use passcli_core::storage::{load_vault, save_vault};
use passcli_core::utils::choose_service_from_list::choose_service_from_list;
use passcli_core::utils::copy_to_clipboard::copy_to_clipboard;
use passcli_core::utils::generate_password::{choose_pass_options, generate_password};
use dialoguer::{Password, Select};

pub fn handle_update(generate: bool) {
    let mut vault = load_vault();

    let service = match choose_service_from_list(Some(&vault)) {
        Some(s) if !s.is_empty() => s,
        _ => {
            println!("No service selected. Update cancelled.");
            std::process::exit(1);
        }
    };

    let accounts = vault.entries.get_mut(&service).unwrap();

    let idx = if accounts.len() == 1 {
        0
    } else {
        let labels: Vec<&str> = accounts.iter().map(|a| a.username.as_str()).collect();
        Select::new()
            .with_prompt("Select account to update")
            .items(&labels)
            .default(0)
            .interact()
            .unwrap()
    };

    let new_password = if generate {
        let opts = choose_pass_options();
        let pwd = generate_password(opts.1, opts.0, opts.2);
        copy_to_clipboard(&pwd);
        pwd
    } else {
        Password::new()
            .with_prompt("Enter new password")
            .interact()
            .unwrap()
    };

    let username = accounts[idx].username.clone();
    accounts[idx].password = new_password;

    save_vault(&vault, None);

    if generate {
        println!(
            "✅ Password for '{}/{}' updated and copied to clipboard.",
            service, username
        );
    } else {
        println!("✅ Password for '{}/{}' updated.", service, username);
    }
}
