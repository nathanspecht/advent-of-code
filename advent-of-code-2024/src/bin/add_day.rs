use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: add-day <file_name>");
        std::process::exit(1);
    }

    let day_name = &args[1];
    let files = vec![
        format!("src/solutions/day{day_name}_a.rs"),
        format!("src/solutions/day{day_name}_b.rs"),
        format!("src/inputs/day{day_name}_a.txt"),
        format!("src/inputs/day{day_name}_b.txt"),
    ];

    let boilerplate_content = r#"pub fn run() { println!("new day"); }"#;

    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .open("src/solutions.rs")
        .expect("Failed to open mod file");

    for file_name in files {
        let path = Path::new(&file_name);
        if !path.exists() {
            let mut file = fs::File::create(&path).expect("Failed to create file");

            if path.extension().unwrap() == "rs" {
                file.write_all(boilerplate_content.as_bytes())
                    .expect("Failed to write to file");

                Command::new("rustfmt")
                    .arg(&file_name)
                    .output()
                    .expect("Failed to run rustfmt");
            } else {
                file.write_all("".as_bytes())
                    .expect("Failed to write to file");
            }

            println!("Created: {:?}", path);
        } else {
            println!("File already exists: {:?}", path);
        }
    }

    writeln!(mod_file, "pub mod day{day_name}_a;", day_name = day_name)
        .expect("Failed to write to file");

    writeln!(mod_file, "pub mod day{day_name}_b;", day_name = day_name)
        .expect("Failed to write to file");
}
