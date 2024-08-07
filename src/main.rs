use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader, Error};

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

fn main() {
    let options = match load_options("options.json") {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("Fehler beim Laden der Optionen: {}", err);
            return;
        }
    };

    println!("Verschlüsselungsprogramm");
    println!("Bitte wählen Sie eine Option:");

    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }

    process_choices(&options);
}

fn process_choices(options: &[String]) {
    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Fehler beim Lesen der Eingabe");

        match choice.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= options.len() => {
                println!("{}", options[num - 1]);
                if num == options.len() {
                    println!("Programm beendet");
                    break;
                }
            }
            _ => println!("Ungültige Auswahl, bitte versuchen Sie es erneut"),
        }
    }
}
