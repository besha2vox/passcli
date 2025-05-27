# 🔐 passcli

A simple and secure **CLI password manager** written in Rust.  
Store, manage, encrypt, and decrypt passwords from your terminal.

## 🚀 Features

- Add, remove, update, and list password entries
- Copy passwords to clipboard
- Encrypt and decrypt your vault with a master password
- Open the vault in any text editor
- Cross-platform (Linux, macOS, Windows)
- No external dependencies (except for optional clipboard support)

## 🛠️ Installation

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/your-username/passcli.git
cd passcli
cargo build --release
```

You’ll find the compiled binary at target/release/passcli.

Optionally, you can move it to a location in your $PATH:

```bash
sudo mv target/release/passcli /usr/local/bin/passcli
```

## 📝 Usage

Run passcli --help to see all available commands and options:

```bash
passcli --help
```

## 🚀 Command Examples

```bash
# Add a new entry manually
passcli add

# Add a new entry with generated password
passcli add --generate

# Copy password to clipboard
passcli copy github

# Encrypt your vault
passcli encrypt

# Decrypt your vault
passcli decrypt

# Open vault with default cat
passcli open

# Open with nano
passcli open nano
```

## 🔐 Vault Format

Passwords are stored in a JSON file (typically in $HOME/.passcli/vault.json) with optional encryption using a user-provided password.

When encrypted, the file contents are AES-encrypted and base64-encoded.

## 🧪 Development

Run with:

```bash
cargo run -- <COMMAND>
```

Example:

```bash
cargo run -- add --generate
```

## 🏗️ Architecture

`passcli` is built as a modular and extensible Rust-based CLI application following a clean separation of concerns. Here's an overview of the core components:

### 🔧 Core Modules

- **`main.rs`**  
  Entry point of the application. It parses CLI arguments using `clap` and delegates execution to the appropriate command handler.

- **`cli.rs`**  
  Defines the structure of the CLI using `clap`’s `Parser` and `Subcommand`. Each subcommand (like `add`, `remove`, `copy`, etc.) is described with its arguments and options.

- **`commands/`**  
  Contains the implementation of each CLI command:
  - `add.rs` – Adding new entries.
  - `remove.rs` – Removing services.
  - `copy.rs` – Copying passwords to clipboard.
  - `open.rs` – Opening the vault with a specified editor.
  - `update.rs` – Updating existing entries.
  - `encrypt.rs` / `decrypt.rs` – Encryption and decryption logic.

- **`crypto.rs`**  
  Handles encryption and decryption using AES-GCM with password-based key derivation.

- **`storage.rs`**  
  Handles loading, saving, and serializing the vault file. Supports conditional encryption logic.

- **`vault.rs`**  
  Defines the `Vault` and `Entry` data structures. Responsible for JSON (de)serialization and logical state management.

## ⚠️ Requirements

- Rust (1.70+ recommended)
- Cargo
- (Optional) `xclip` or `pbcopy` for clipboard support on Linux/macOS

## 📁 Vault Format

```json
{
  "entries": {
    "github": {
      "password": "hunter2",
      "encrypted": false
    }
  },
  "encrypted": false
}
```

When encryption is enabled, the entire file is encrypted.

Only decrypted vaults are shown in plain JSON format.

## 🔐 Security Considerations

- Vault is optionally encrypted using AES-GCM with a password-derived key.
- Passwords are not stored in plaintext when encryption is enabled.
- Password input is hidden from the terminal using `rpassword`.
- For maximum safety, ensure your vault is always encrypted and use a strong password.
- Encrypted vaults are stored as Base64-encoded binary files.
- Password-based key derivation ensures that only users with the correct password can decrypt the vault.

## 🧪 Testing

To run tests:

```bash
cargo test
```

Ensure your tests cover edge cases such as invalid input, corrupted vault files, and encryption/decryption failures.

## 📦 Packaging

To build the release version:

```bash
cargo build --release
```

## 🧱 Roadmap

- [ ] Implement automatic password expiration warnings
- [ ] Add search/filter command
- [ ] Export vault to CSV
- [ ] Sync vault to cloud (optional backend)
- [ ] UI with TUI or Web front-end

## 🙋‍♂️ Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Create a new Pull Request

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
