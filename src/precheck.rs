use spinoff::{spinners, Color, Spinner};
use std::process::exit;
use std::process::Command;
use std::process::Stdio;

pub fn precheck() {
    //check if NPM is installed
    let npm_check = Command::new("npm")
        .args(["-v"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if npm_check.is_err() || !npm_check.unwrap().success() {
        println!("✖ NPM is not installed");
        exit(1);
    }
    //check if Cargo is installed
    let cargo_check = Command::new("cargo")
        .args(["--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if cargo_check.is_err() || !cargo_check.unwrap().success() {
        println!("✖ Cargo is not installed");
        exit(1);
    }
    //check if Vite is installed
    let vite_check = Command::new("npm")
        .args(["list", "-g", "create-vite"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    let vite_outdated = Command::new("npm")
        .args(["outdated", "-g", "create-vite"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if vite_check.is_err()
        || !vite_check.unwrap().success()
        || vite_outdated.is_ok() && vite_outdated.unwrap().success()
    {
        let mut spinner = Spinner::new(spinners::Dots, "Installing Dependencies", Color::White);
        // show a spinner while npm installs/updates create-vite
        let install_status = Command::new("npm")
            .arg("install")
            .arg("-g")
            .arg("create-vite@latest")
            .arg("--force")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to run npm install");

        if install_status.success() {
            spinner.clear();
        } else {
            spinner.stop_and_persist("✖", "Installation failed");
            exit(1);
        }
    }
    // check if Tailwind is installed
    let tailwind_check = Command::new("npm")
        .args(["list", "-g", "tailwindcss"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    let tailwind_outdated = Command::new("npm")
        .args(["outdated", "-g", "tailwindcss"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if tailwind_check.is_err()
        || !tailwind_check.unwrap().success()
        || tailwind_outdated.is_ok() && tailwind_outdated.unwrap().success()
    {
        let mut spinner = Spinner::new(spinners::Dots, "Installing Dependencies", Color::White);
        let install_status = Command::new("npm")
            .arg("install")
            .arg("-g")
            .arg("tailwindcss@latest")
            .arg("--force")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to run npm install");

        if install_status.success() {
            spinner.clear();
        } else {
            spinner.stop_and_persist("✖", "Tailwind installation failed");
            exit(1);
        }
    }
}
