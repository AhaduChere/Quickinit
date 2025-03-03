use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn make_c(input: &str) {
    let project_name = input.to_string();

    // create project directory
    fs::create_dir(&project_name).unwrap();
    let file_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(&project_name);

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
        "TARGET_DIR := $(PROJECT_ROOT)/build",
        "TARGET := $(TARGET_DIR)/$(PROJECT_NAME)",
        "",
        "# Find all C source files.",
        "SRCS := $(wildcard src/*.c)",
        "",
        "# Default target: build the executable into target/debug/<project_name>",
        "all: $(TARGET)",
        "	@echo 'Built executable: $(TARGET)'",
        "",
        "$(TARGET): $(SRCS) | $(TARGET_DIR)",
        "	gcc $(SRCS) -o $(TARGET)",
        "",
        "$(TARGET_DIR):",
        "	mkdir -p $(TARGET_DIR)",
        "",
        "clean:",
        "	rm -r $(TARGET_DIR)",
    ];
    for makefile_text in makefile_text.iter() {
        writeln!(makefile, "{}", makefile_text).expect("Failed to write to Makefile");
    }

    // create build folder
    let build_path = file_path.join("build");
    fs::create_dir_all(&build_path).expect("Failed to create 'build' directory");

    // create lib folder
    let lib_path = file_path.join("lib");
    fs::create_dir_all(&lib_path).expect("Failed to create 'lib' directory");

    // create src folder
    let src_path = file_path.join("src");
    fs::create_dir_all(&src_path).expect("Failed to create 'src' directory");

    // create main.c file
    let file_in_src = src_path.join("main.c");
    let mut main_c = File::create(&file_in_src).expect("Failed to create file 'main.c'");
    let main_c_text = [
        "#include <stdio.h>",
        "",
        "int main() {",
        "",
        "  printf(\"Hello World\");",
        "",
        "  return 0;",
        "}",
    ];
    for main_c_text in main_c_text.iter() {
        writeln!(main_c, "{}", main_c_text).expect("Failed to write to Makefile");
    }

    //create utils.c
    let file_in_src = src_path.join("utils.c");
    File::create(&file_in_src).expect("Failed to create file 'utils.c'");

    // create include folder
    let include_path = file_path.join("Include");
    fs::create_dir_all(&include_path).expect("Failed to create 'Include' directory");

    // create utils.h file
    let file_in_include = include_path.join("utils.h");
    File::create(&file_in_include).expect("Failed to create file 'utils.h'");

    // initialize git 
    Command::new("git")
        .args(["init"])
        .args([file_path])
        .output()
        .expect("Error creating git folder");

    println!("Project {} ready.", project_name);
}
