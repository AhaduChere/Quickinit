use dialoguer::Select;
use spinoff::{spinners, Color, Spinner};
use std::fs;
use std::process::{Command, Stdio};

pub fn react(input: &str) {
    let project_name = input.to_string().to_lowercase();

    let mut spinner = Spinner::new(spinners::Dots, "Creating directory...", Color::White);
    // initialize project
    let status = Command::new("npm")
        .arg("create")
        .arg("vite@latest")
        .arg(&project_name)
        .arg("--")
        .arg("--template")
        .arg("react")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect(" ✖ Failed to execute npm command");

    if status.success() {
        spinner.stop_and_persist(" 󰸞", "Directory created successfully!");
    } else {
        spinner.stop_and_persist(" ✖", "Directory creation failed!");
    }
    println!("Tailwindcss?");
    let tw_opts = vec!["Yes", "No"];
    let tailwind = Select::new().items(&tw_opts).default(0).interact().unwrap();
    let tw_choices = tw_opts[tailwind];
    println!("{}", tw_choices);
    let project_path = format!("./{}", project_name);

    match tw_choices {
        "Yes" => {
            let mut spinner =
                Spinner::new(spinners::Dots, "Installing Tailwindcss...", Color::White);

            let status = Command::new("npm")
                .current_dir(&project_path)
                .arg("install")
                .arg("-D")
                .arg("tailwindcss")
                .arg("postcss")
                .arg("autoprefixer")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .expect(" ✖ Failed to execute npm command");

            if status.success() {
                spinner.stop_and_persist(" 󰸞", "Tailwindcss installed successfully!");
            } else {
                spinner.stop_and_persist(" ✖", "Tailwindcss failed to install");
            }

            let config_content = r#"module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: { extend: {} },
  plugins: [],
};"#;

            std::fs::write(
                format!("{}/tailwind.config.js", project_path),
                config_content,
            )
            .expect("Failed to write Tailwind config");

            let css_content = "@tailwind base;\n@tailwind components;\n@tailwind utilities;\n";
            std::fs::write(format!("{}/src/index.css", project_path), css_content)
                .expect("Failed to update index.css");

            // Clean up files
            let _ = fs::remove_file(format!("{}/src/App.css", project_path));
            let _ = fs::remove_dir_all(format!("{}/src/assets", project_path));
            let _ = fs::remove_file(format!("{}/public/vite.svg", project_path));

            // Replace App.jsx content
            let app_content = r#"function App() {
  return (
    <>



    </>
  )
}

export default App
"#;
            fs::write(format!("{}/src/App.jsx", project_path), app_content)
                .expect("Failed to update App.jsx");

            // Replace index.html content
            let html_content = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" /> 
    <link rel="icon" type="image/svg+xml" href="" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title></title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.jsx"></script>
  </body>
</html>
"#;
            fs::write(format!("{}/index.html", project_path), html_content)
                .expect("Failed to update index.html");
        }

        "No" => {
            // Clean up files
            let _ = fs::remove_file(format!("{}/src/App.css", project_path));
            let _ = fs::remove_dir_all(format!("{}/src/assets", project_path));
            let _ = fs::remove_file(format!("{}/public/vite.svg", project_path));

            // Replace App.jsx content
            let app_content = r#"function App() {
  return (
    <>



    </>
  )
}

export default App
"#;
            fs::write(format!("{}/src/App.jsx", project_path), app_content)
                .expect("Failed to update App.jsx");

            // Replace index.html content
            let html_content = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" /> 
    <link rel="icon" type="image/svg+xml" href="" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title></title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.jsx"></script>
  </body>
</html>
"#;
            fs::write(format!("{}/index.html", project_path), html_content)
                .expect("Failed to update index.html");

            let css_content = "";
            std::fs::write(format!("{}/src/index.css", project_path), css_content)
                .expect("Failed to update index.css");
        }
        _ => println!("Unknown choice"),
    }
    println!("");
    println!("Run the following:");
    println!("cd {} && npm install", project_name);
}
