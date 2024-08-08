use std::sync::{Arc, Mutex};

pub fn sha256(text: Arc<Mutex<String>>) {
    let text = text.lock().unwrap();
    println!("sha256 ausgewählt.");
    println!("Originaltext: {}", *text);
    // Implementiere die sha256 Logik hier
    // Beispiel: Verschiebe jeden Buchstaben um 3 Positionen
    let encrypted: String = text.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            (first + (c as u8 - first + 3) % 26) as char
        } else {
            c
        }
    }).collect();
    println!("Verschlüsselter Text: {}", encrypted);
}