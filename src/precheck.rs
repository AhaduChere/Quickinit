use spinoff::{spinners, Color, Spinner};
use std::process::{exit, Command, Stdio};

pub fn precheck() {
    let mut spinner = Spinner::new(spinners::Dots, "Checking Dependencies", Color::White);

    // Check if Cargo is installed
    let cargo = Command::new("cargo")
        .arg("-V")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    // Check if npm is installed
    let npm = Command::new("npm")
        .arg("-v")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    // Check if create-vite is installed
    let create_vite = Command::new("npm")
        .args(["list", "-g", "create-vite", "--depth=0"])
        .output()
        .ok()
        .and_then(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .find(|line| line.contains("create-vite@"))
                .and_then(|line| line.split('@').last())
                .map(|v| v.trim().to_string())
        })
        .and_then(|installed| {
            Command::new("npm")
                .args(["view", "create-vite", "version"])
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .map(|latest| installed == latest)
        })
        .unwrap_or(false);

    // Check if Tailwind CSS is installed
    let tailwindcss = Command::new("npm")
        .args(["list", "-g", "tailwindcss", "--depth=0"])
        .output()
        .ok()
        .and_then(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .find(|line| line.contains("tailwindcss@"))
                .and_then(|line| line.split('@').last())
                .map(|v| v.trim().to_string())
        })
        .and_then(|installed| {
            Command::new("npm")
                .args(["view", "tailwindcss", "version"])
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .map(|latest| installed == latest)
        })
        .unwrap_or(false);

    if create_vite && tailwindcss && cargo && npm {
        spinner.success("All dependencies installed.");
    } else {
        spinner.fail("Some dependencies are missing. Please install them before proceeding:");
        if !cargo {
            eprintln!("Missing Dependency: Cargo.");
        }
        if !npm {
            eprintln!("Missing Dependency: NPM.");
        }
        if !create_vite {
            eprintln!("Missing or Outdated Dependency: create-vite.");
        }
        if !tailwindcss {
            eprintln!("Missing or Outdated Dependency: Tailwind CSS.");
        }
        exit(1);
    }
}
