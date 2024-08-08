use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn xor_cipher(text: &str, key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    text.bytes()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
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

    let encrypted_text = xor_cipher(text, key);
    let encrypted_hex: String = encrypted_text.iter().map(|b| format!("{:02x}", b)).collect();
    println!("Encrypted text (hex): {}", encrypted_hex);
}

pub fn xor_console_wrapper(text: Arc<Mutex<String>>) {
    let text_lock = text.lock().unwrap().clone();

    let mut key_input = String::new();
    print!("Enter key: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut key_input).unwrap();
    let key = key_input.trim();

    let encrypted_text = xor_cipher(&text_lock, key);
    let encrypted_hex: String = encrypted_text.iter().map(|b| format!("{:02x}", b)).collect();
    println!("\"{}\" -> \"XOR\"({}) = \"{}\"", text_lock, key, encrypted_hex);
}