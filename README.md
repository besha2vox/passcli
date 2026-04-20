# 🔐 passcli

A secure password manager written in Rust — available as both a **CLI tool** and a **desktop GUI** (Tauri + Nuxt 3).

## Features

- Multiple accounts per service (username + password per entry)
- Add, remove, update, list password entries
- Copy passwords to clipboard
- AES-256-GCM vault encryption with a master password
- Generate strong random passwords (length, case, special chars)
- Desktop GUI with service list, account detail panel, and password generator
- Cross-platform (Linux, macOS, Windows)

## Project Structure

```text
passcli/
├── core/          # passcli-core — shared library (models, crypto, storage, utils)
├── cli/           # passcli-cli  — terminal binary
└── gui/           # PassHolder desktop app
    ├── src-tauri/ # Tauri v2 backend (Rust, bridges core → frontend)
    ├── components/ # Vue 3 components
    ├── composables/ # useVault, useFavorites
    └── pages/      # Nuxt 3 pages
```

## Installation

Requires [Rust](https://www.rust-lang.org/tools/install) 1.70+ and Cargo.

```bash
git clone https://github.com/your-username/passcli.git
cd passcli
```

### CLI

```bash
cargo build --release -p passcli-cli
# binary → target/release/passcli
sudo mv target/release/passcli /usr/local/bin/passcli
```

### Desktop GUI

Requires [Node.js](https://nodejs.org/) 18+ and system WebKit2GTK (Linux).

```bash
cd gui
npm install
npm run tauri:build
# AppImage/deb → gui/src-tauri/target/release/bundle/
```

## CLI Usage

```bash
passcli --help
```

```bash
# Add a new account (prompts for service, username, password)
passcli add

# Add with generated password
passcli add --generate

# List all services and accounts
passcli list

# Copy password to clipboard
# One account → copies immediately; multiple → prompts to select by username
passcli copy github

# Update an existing account's password
passcli update
passcli update --generate

# Remove an account
# One account → removes the service; multiple → prompts to select or remove all
passcli remove github

# Vault encryption
passcli encrypt
passcli decrypt
passcli status

# Open vault file in editor
passcli open
passcli open nano
```

## Vault Format

Located at `~/.config/passcli/vault.json`. Each service holds a list of accounts:

```json
{
  "entries": {
    "github": [
      { "username": "john@gmail.com", "password": "hunter2" },
      { "username": "work@company.com", "password": "s3cr3t" }
    ],
    "twitter": [
      { "username": "john_doe", "password": "p@ssw0rd" }
    ]
  },
  "encrypted": false
}
```

When encrypted, the entire JSON is AES-256-GCM encrypted and Base64-encoded.

## Security

- AES-256-GCM encryption, key derived via PBKDF2-SHA256 (100 000 iterations)
- Wire format: `salt(16 B) || nonce(12 B) || ciphertext` → Base64
- Password input always via `rpassword` (hidden from terminal)
- Plaintext passwords never printed or logged

## Roadmap

- [ ] Automatic password expiration warnings
- [ ] Search/filter command in CLI
- [ ] Export vault to CSV
- [ ] Edit account username in GUI

## License

MIT — see [LICENSE](LICENSE).
