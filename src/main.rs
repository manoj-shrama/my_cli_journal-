
use std::fs::OpenOptions;
use std::io::{self,Write};

fn main () {
    println!("welcome to your cli journal ");
    print!("write our entry : ");
    io::stdout().flush().unwrap();

let mut input = String::new();
io::stdin()
.read_line(&mut input)
.expect("failed to read line");
let cleaned_input = input.trim();

let mut file = OpenOptions::new()
.append(true)
.create(true)
.open("journal.txt")
.expect("failed to open or create file");
writeln!(file,"{}",cleaned_input).expect("failed ot write to file");
println!("\n success ! your entry has been saved to journal.txt ");

}