use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn make_rust(input: &str) {
    let project_name = input.to_string();

    // create project directory
    Command::new("cargo")
        .args(["new"])
        .args([&project_name])
        .status()
        .expect("Error executing Cargo new");
    let file_path = std::env::current_dir().unwrap().join(&project_name);

    // create Makefile
    let makefile_path = file_path.join("Makefile");
    let mut makefile = File::create(&makefile_path).expect("Failed to create Makefile");
    let makefile_text = [
        "# Try to detect the git root directory; if not found, default to current directory.",
        "GIT_ROOT := $(shell git rev-parse --show-toplevel 2>/dev/null)",
        "ifeq ($(GIT_ROOT),)",
        "	PROJECT_ROOT := $(CURDIR)",
        "else",
        "	PROJECT_ROOT := $(GIT_ROOT)",
        "endif",
        "",
        "# Use the folder name at the project root as the project name.",
        "PROJECT_NAME := $(notdir $(PROJECT_ROOT))",
        "TARGET_DIR := $(PROJECT_ROOT)/target/debug",
        "",
        "# Default target: build the project using Cargo.",
        "all:",
        "	cargo build",
        "	@echo 'Built executable: $(TARGET_DIR)/$(PROJECT_NAME)'",
        "",
        "# Clean build artifacts.",
        "clean:",
        "	cargo clean",
    ];
    for makefile_text in makefile_text.iter() {
        writeln!(makefile, "{}", makefile_text).expect("Failed to write to Makefile");
    }
    println!("Project {} ready.", &project_name);
}
