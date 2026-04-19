use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Vault {
    pub entries: HashMap<String, Vec<Account>>,
    pub encrypted: bool,
}
