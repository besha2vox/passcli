pub fn copy_to_clipboard(password: &str) {
    let mut clipboard = arboard::Clipboard::new().expect("Failed to access clipboard");
    clipboard
        .set_text(password.to_string())
        .expect("Error while copying to clipboard");
}
