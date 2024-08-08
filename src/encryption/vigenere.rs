use std::sync::{Arc, Mutex};

pub fn vigenere(text: Arc<Mutex<String>>) {
    let text = text.lock().unwrap();
    println!("vigenere ausgewählt.");
    println!("Originaltext: {}", *text);
    // Implementiere die vigenere Logik hier
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