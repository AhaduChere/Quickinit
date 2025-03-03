use dialoguer::Select;
use std::io;

mod make_c; 
use make_c::make_c; 

mod make_rust;
use make_rust::make_rust;

mod make_javascript;
use make_javascript::make_javascript;

fn main() {
    // gets Project Name
    println!("Enter Project Name:");
    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line");

    // Trim newline and convert to String
    let project_name = project_name.trim().to_string(); 

    // gets programming language
    println!("Choose your language:");
    let items = vec!["C", "Rust", "Javascript"];
    let selection = Select::new().items(&items).default(0).interact().unwrap();
    let choice = items[selection];
    println!("{}", choice);

    match choice {
        "C" => make_c(&project_name),
        "Rust" => make_rust(&project_name),
        "Javascript" => make_javascript(&project_name),
        _ => println!("Unknown choice"),
    }
}

