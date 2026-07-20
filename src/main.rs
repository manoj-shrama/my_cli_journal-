use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if "read" command was passed
    if args.len() > 1 && args[1] == "read" {
        println!("--- 📖 Reading Your Journal Entries ---");
        
        match fs::read_to_string("journal.txt") {
            Ok(content) => {
                if content.trim().is_empty() {
                    println!("Your journal is empty! Write something first.");
                } else {
                    println!("{}", content);
                }
            }
            Err(_) => {
                println!("No journal file found yet. Write an entry first to create it!");
            }
        }
        return;
    }

    // Default "Write" Mode
    println!("--- 🚀 Welcome to your CLI Journal ---");
    print!("Write your entry: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let cleaned_input = input.trim();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("journal.txt")
        .expect("Failed to open or create file");

    writeln!(file, "{}", cleaned_input).expect("Failed to write to file");

    println!("\n✅ Success! Your entry has been saved to journal.txt");
}