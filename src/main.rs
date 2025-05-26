mod cli;
mod commands;
pub mod crypto;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use commands::*;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { generate } => {
            add::handle_add(generate);
        }
        Commands::Remove { service } => {
            remove::handle_remove(&service);
        }
        Commands::Copy { service } => {
            copy::handle_copy(&service);
        }
        Commands::Open { editor } => {
            open::handle_open(&editor);
        }
        Commands::List => {
            list::handle_list();
        }
        Commands::Update { service } => {
            update::handle_update(&service);
        }
        Commands::Path => {
            let path = storage::get_vault_path();
            println!("Vault path: {}", path.display());
        }
        Commands::Encrypt => {
            encrypt::handle_encrypt();
        }
        Commands::Decrypt => {
            encrypt::handle_decrypt();
        }
        Commands::Status => {
            status::handle_status();
        }
    }
}
