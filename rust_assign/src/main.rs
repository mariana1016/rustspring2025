use std::fs::File;
use std::io::{self, prelude::*, BufRead, BufReader, Write};

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn reading_from_console() -> io::Result<()> {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush()?; // Ensures the prompt appears immediately
    io::stdin().read_line(&mut buffer)?;
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("How old are you? ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;
    
    let age: u32 = buffer.trim().parse().unwrap_or_else(|_| {
        eprintln!("Invalid age input. Defaulting to 0.");
        0
    });

    let person = Person { name, age };
    println!("Hi {}, you are {} years old!", person.name, person.age);

    Ok(())
}

#[derive(Debug)]
struct Config {
    username: String,
    api_key: String,
    port: u16,
}

impl Config {
    fn from_file(path: &str) -> io::Result<Config> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let username = lines.next().unwrap_or(Ok("".to_string()))?;
        let api_key = lines.next().unwrap_or(Ok("".to_string()))?;
        let port = lines
            .next()
            .unwrap_or(Ok("8080".to_string()))?
            .parse()
            .unwrap_or_else(|_| {
                eprintln!("Invalid port in file. Defaulting to 8080.");
                8080
            });

        Ok(Config { username, api_key, port })
    }
}

fn reading_from_file() -> io::Result<()> {
    match Config::from_file("config.txt") {
        Ok(config) => {
            println!("Config Loaded:");
            println!("Username: {}", config.username);
            println!("API Key: {}", config.api_key);
            println!("Port: {}", config.port);
        }
        Err(e) => eprintln!("Error reading config file: {}", e),
    }
    Ok(())
}

fn main() -> io::Result<()> {
    reading_from_console()?;
    reading_from_file()?;
    Ok(())
}


