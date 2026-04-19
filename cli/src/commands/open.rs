use passcli_core::storage::get_vault_path;
use std::process::Command;

pub fn handle_open(editor: &str) {
    let path = get_vault_path();

    let status = Command::new(editor).arg(&path).status();

    match status {
        Ok(s) if s.success() => {
            println!("📂 Opened vault with '{}'.", editor);
        }
        Ok(_) => {
            println!("⚠️ Editor '{}' exited with a non-zero status.", editor);
        }
        Err(e) => {
            println!("❌ Failed to launch editor '{}': {}", editor, e);
        }
    }
}
