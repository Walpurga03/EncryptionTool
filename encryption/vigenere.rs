use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn vigenere_cipher(text: &str, key: &str) -> String {
    let key = key.chars().cycle();
    text.chars()
        .zip(key)
        .map(|(c, k)| {
            if c.is_ascii_alphabetic() {
                let a = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let c = c as u8;
                let k = k.to_ascii_uppercase() as u8;
                ((c - a + (k - b'A')) % 26 + a) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn run_console_version() {
    let mut text = String::new();
    let mut key = String::new();

    print!("Enter text to encrypt: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text).unwrap();
    let text = text.trim();

    print!("Enter key: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key).unwrap();
    let key = key.trim();

    if !key.chars().all(|c| c.is_ascii_alphabetic()) {
        println!("Key must contain only letters.");
        return;
    }

    let encrypted_text = vigenere_cipher(text, key);
    println!("Encrypted text: {}", encrypted_text);
}

pub fn vigenere_console_wrapper(text: Arc<Mutex<String>>) {
    let text_lock = text.lock().unwrap().clone();

    let mut key_input = String::new();
    print!("Enter key: ");
    io::stdout().flush().unwrap();

    loop {
        io::stdin().read_line(&mut key_input).unwrap();
        let key = key_input.trim();

        if !key.chars().all(|c| c.is_ascii_alphabetic()) {
            println!("Key must contain only letters. Please enter a valid key:");
            key_input.clear(); // Clear the input for the next attempt
            continue;
        }

        let encrypted_text = vigenere_cipher(&text_lock, key);
        println!("\"{}\" -> \"Vigenere\"({}) = \"{}\"", text_lock, key, encrypted_text);
        break;
    }
}