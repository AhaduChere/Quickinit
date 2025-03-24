use dialoguer::Select;

mod vue;
use vue::vue;

mod react;
use react::react;

pub fn make_js(input: &str) {

    println!("Choose your framework:");
    let items = vec!["Vue", "React"];
    let framework = Select::new().items(&items).default(0).interact().unwrap();
    let choice = items[framework];
    println!("{}", choice);

    match choice {
        "Vue" => {
            vue(input);
        }
        "React" => {
            react(input);
        }
        _ => println!("Unknown choice"),
    }
}
