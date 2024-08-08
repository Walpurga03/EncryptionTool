use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn caesar_cipher(text: &str, shift: i32) -> String {
    let shift = shift % 26;
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_uppercase() { 'A' } else { 'a' };
                let shifted = ((c as u8 - first as u8) as i32 + shift) % 26;
                (shifted + first as i32) as u8 as char
            } else {
                c
            }
        })
        .collect()
}

pub fn run_console_version() {
    let mut text = String::new();
    let mut shift = String::new();

    print!("Enter text to encrypt: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text).unwrap();
    let text = text.trim();

    print!("Enter shift amount: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut shift).unwrap();
    let shift: i32 = shift.trim().parse().unwrap();

    let encrypted_text = caesar_cipher(text, shift);
    println!("Encrypted text: {}", encrypted_text);
}

pub fn caesar_console_wrapper(text: Arc<Mutex<String>>) {
    let text_lock = text.lock().unwrap().clone();

    let mut shift_input = String::new();
    print!("Enter shift amount: ");
    io::stdout().flush().unwrap();

    let shift: i32;
    loop {
        io::stdin().read_line(&mut shift_input).unwrap();
        shift = match shift_input.trim().parse() {
            Ok(num) if num >= 1 && num <= 25 => num,
            _ => {
                println!("A valid shift value between 1 and 25:");
                shift_input.clear(); // Clear the input for the next attempt
                continue;
            }
        };
        let encrypted_text = caesar_cipher(&text_lock, shift);
        println!("\"{}\" -> \"Caesar\"({}) = \"{}\"", text_lock, shift, encrypted_text);
        break;
    }
}