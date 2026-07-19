use std::io::{self,Write};
fn main () {
    println!("welcome to your cli journal ");
    print!("write our entry");
    io::stdout().flush().unwrap();

let mut input = String::new();
io::stdin()
.read_line(&mut input)
.expect("failed to read line");
let cleaned_input = input.trim();
println!("\nSaved text : \"{}\"",cleaned_input);

}