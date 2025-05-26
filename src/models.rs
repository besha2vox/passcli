use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntry {
    pub password: String,
    pub encrypted: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Vault {
    pub entries: HashMap<String, PasswordEntry>,
    pub encrypted: bool,
}
