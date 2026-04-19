use crate::{models::Vault, storage::get_vault_path};
use serde_json::from_slice;
use std::fs;

pub fn is_vault_encrypted() -> bool {
    let path = get_vault_path();

    if let Ok(data) = fs::read(&path) {
        from_slice::<Vault>(&data).is_err()
    } else {
        false
    }
}
