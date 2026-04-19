use passcli_core::storage::load_vault;
use passcli_core::utils::copy_to_clipboard::copy_to_clipboard;
use passcli_core::utils::generate_password::{choose_pass_options, generate_password};
use passcli_core::utils::save_password::save_password;
use dialoguer::{Input, Password};

pub fn handle_add(generate: bool) {
    if generate {
        generate_and_add();
    } else {
        manual_add();
    }
}

fn prompt_service_and_username() -> (String, String) {
    let service: String = Input::new()
        .with_prompt("Enter service name")
        .interact_text()
        .unwrap();

    let username: String = Input::new()
        .with_prompt("Enter username")
        .interact_text()
        .unwrap();

    check_duplicate(&service, &username);
    (service, username)
}

fn manual_add() {
    let (service, username) = prompt_service_and_username();

    let password: String = Password::new()
        .with_prompt("Enter password")
        .interact()
        .unwrap();

    save_password(service.clone(), username.clone(), password, None);
    println!("✅ Account '{}' for '{}' saved.", username, service);
}

fn generate_and_add() {
    let (service, username) = prompt_service_and_username();

    let opts = choose_pass_options();
    let generated = generate_password(opts.1, opts.0, opts.2);

    save_password(service.clone(), username.clone(), generated.clone(), None);
    copy_to_clipboard(&generated);
    println!(
        "✅ Account '{}' for '{}' saved and password copied to clipboard.",
        username, service
    );
}

fn check_duplicate(service: &str, username: &str) {
    let vault = load_vault();
    if let Some(accounts) = vault.entries.get(service) {
        if accounts.iter().any(|a| a.username == username) {
            println!(
                "❌ Account '{}' for '{}' already exists. Use 'update' to change it.",
                username, service
            );
            std::process::exit(1);
        }
    }
}
