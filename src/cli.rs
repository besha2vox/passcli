use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "passcli")]
#[command(about = "CLI password manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new service or entry to the vault
    Add {
        /// Generate a random password for the new entry
        #[arg(short, long, help = "Generate a random password")]
        generate: bool,
    },

    /// Remove a service or entry from the vault
    Remove {
        /// The service or entry to remove
        #[arg(help = "Name of service to remove")]
        service: String,
    },

    /// Copy the password of a service to the clipboard
    Copy {
        /// The service whose password to copy
        #[arg(help = "Name of service to copy password for")]
        service: String,
    },

    /// Open the vault in a text editor
    Open {
        /// The text editor to use for opening the vault
        #[arg(default_value = "cat", help = "Text editor to open the vault")]
        editor: String,
    },

    /// List all services or entries in the vault
    List,

    /// Update an existing service or entry in the vault
    Update {
        /// Generate a new random password for the entry
        #[arg(short, long, help = "Generate a new random password")]
        generate: bool,
    },

    /// Display the path to the vault file
    Path,

    /// Encrypt the vault
    Encrypt,

    /// Decrypt the vault
    Decrypt,

    /// Display the status of the vault
    Status,
}
