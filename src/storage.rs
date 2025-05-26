use crate::crypto::{decrypt, encrypt};
use crate::models::Vault;
use dirs::config_dir;
use rpassword::read_password;
use std::fs;
use std::path::PathBuf;

pub fn get_vault_path() -> PathBuf {
    let mut path = config_dir().expect("Failed to find config directory");
    path.push("passcli");
    fs::create_dir_all(&path).unwrap();
    path.push("vault.json");
    path
}

// Loads the vault from the file system, decrypting it if necessary
pub fn load_vault() -> Vault {
    let path = get_vault_path();

    if !path.exists() {
        return Vault::default();
    }

    let data = fs::read_to_string(&path).expect("Failed to read vault");

    if let Ok(vault) = serde_json::from_str::<Vault>(&data) {
        if vault.encrypted {
            println!("Vault is encrypted. Please enter your password:");
            let password = read_password().expect("Failed to read password");

            match decrypt(&password, &data) {
                Ok(decrypted_bytes) => {
                    let decrypted_str = String::from_utf8(decrypted_bytes)
                        .expect("Decrypted data is not valid UTF-8");
                    serde_json::from_str::<Vault>(&decrypted_str)
                        .expect("Failed to parse decrypted vault")
                }
                Err(e) => {
                    eprintln!("Error decrypting vault: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            vault
        }
    } else {
        println!("Vault seems encrypted or corrupted. Please enter password:");
        let password = read_password().expect("Failed to read password");
        match decrypt(&password, &data) {
            Ok(decrypted_bytes) => {
                let decrypted_str =
                    String::from_utf8(decrypted_bytes).expect("Decrypted data is not valid UTF-8");
                serde_json::from_str::<Vault>(&decrypted_str)
                    .expect("Failed to parse decrypted vault")
            }
            Err(e) => {
                eprintln!("Error decrypting vault: {}", e);
                std::process::exit(1);
            }
        }
    }
}

// Saves the vault to the file system, encrypting it if necessary
pub fn save_vault(vault: &Vault, password: Option<&str>) {
    let path = get_vault_path();

    if vault.encrypted {
        let password = password.expect("Password required for encrypted vault");
        let vault_json = serde_json::to_string_pretty(&vault).expect("Serialization error");
        let encrypted_data = encrypt(password, vault_json.as_bytes());
        fs::write(path, encrypted_data).expect("Failed to write encrypted vault");
    } else {
        let vault_json = serde_json::to_string_pretty(vault).expect("Serialization error");
        fs::write(path, vault_json).expect("Failed to write vault");
    }
}

// Reads the vault file as raw bytes without decryption
pub fn read_vault_raw() -> Vec<u8> {
    let path = get_vault_path();
    fs::read(path).expect("Failed to read vault file")
}

pub fn decrypt_vault(password: &str, encrypted_data: &[u8]) -> Result<Vault, String> {
    let data_str = std::str::from_utf8(encrypted_data).expect("Data is not valid UTF-8");
    match decrypt(password, data_str) {
        Ok(decrypted_bytes) => {
            let decrypted_str = String::from_utf8(decrypted_bytes)
                .map_err(|_| "Decrypted data is not valid UTF-8".to_string())?;
            serde_json::from_str::<Vault>(&decrypted_str)
                .map_err(|_| "Failed to parse decrypted vault".to_string())
        }
        Err(_) => Err("Incorrect password or corrupted data".to_string()),
    }
}
