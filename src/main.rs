use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error, stdin, stdout, Write};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

mod encryption;

#[derive(Deserialize)]
struct Config {
    options: Vec<String>,
}

fn load_options(file_path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config.options)
}

fn run_console_app() {
    let options = match load_options("options.json") {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("Fehler beim Laden der Optionen: {}", err);
            return;
        }
    };

    let text = Arc::new(Mutex::new(String::new()));
    {
        let mut text_lock = text.lock().unwrap();
        println!("Please enter the text to be encrypted:");
        stdin().read_line(&mut text_lock).unwrap();
        *text_lock = text_lock.trim().to_string();
    }

    let mut methods: HashMap<&str, fn(Arc<Mutex<String>>)> = HashMap::new();
    methods.insert("Caesar", encryption::caesar::caesar_console_wrapper);
    methods.insert("Vigenere", encryption::vigenere::vigenere_console_wrapper);
    methods.insert("XOR", encryption::xor::xor_console_wrapper);
    methods.insert("Blowfish", encryption::blowfish::blowfish);
    methods.insert("SHA-256", encryption::sha256::sha256);
    methods.insert("AES", encryption::aes::aes);
    methods.insert("RSA", encryption::rsa::rsa);
    methods.insert("RIPEMD-160", encryption::ripemd160::ripemd160);

    loop {
        println!("Please choose an option:");

        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        print!("Your choice: ");
        stdout().flush().unwrap();

        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        if choice == 0 || choice > options.len() {
            println!("Ungültige Wahl, bitte eine gültige Option wählen.");
            continue;
        }

        let method_name = options[choice - 1].as_str();
        if method_name == "Beenden" {
            println!("Programm wird beendet.");
            break;
        }

        match methods.get(method_name) {
            Some(method) => method(Arc::clone(&text)),
            None => println!("Unbekannte Option."),
        }
    }
}

#[wasm_bindgen]
pub fn wasm_entry_point() {
    // WebAssembly Entry Point
}

fn main() {
    if cfg!(target_arch = "wasm32") {
        wasm_entry_point();
    } else {
        run_console_app();
    }
}
