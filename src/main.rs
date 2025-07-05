use dialoguer::Select;
use std::io;
use std::process;

mod make_rust;
use make_rust::make_rust;

mod precheck;
use precheck::precheck;

mod make_js;
use make_js::make_js;

fn main() {
    precheck();

    // gets Project Name
    println!("Enter Project Name:");
    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line");

    // Trim newline, convert to String, and checks if project name already exists
    let project_name = project_name.trim().to_string();
    let test_dir = std::env::current_dir().unwrap().join(&project_name);

    if test_dir.exists() {
        println!("✖ A directory with that name already exists here");
        process::exit(1);
    } else {
    }

    // gets programming language
    println!("Choose your language:");
    let items = vec!["Rust", "Javascript"];
    let selection = Select::new().items(&items).default(0).interact().unwrap();
    let choice = items[selection];
    println!("{}", choice);

    match choice {
        "Rust" => make_rust(&project_name),
        "Javascript" => make_js(&project_name),
        _ => println!("Unknown choice"),
    }
}
