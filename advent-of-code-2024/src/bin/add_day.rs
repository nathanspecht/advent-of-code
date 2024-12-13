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

    let dirs = vec!["solutions", "inputs"];
    let parts = vec!["a", "b"];

    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .open("src/solutions.rs")
        .expect("Failed to open mod file");

    for dir in dirs {
        for part in &parts {
            let ext = match dir {
                "solutions" => "rs",
                "inputs" => "txt",
                _ => "",
            };
            let file_name = format!("src/{dir}/day{day_name}_{part}.{ext}",);
            let path = Path::new(&file_name);
            if !path.exists() {
                let mut file = fs::File::create(&path).expect("Failed to create file");

                if dir == "solutions" {
                    let boilerplate_content = format!(
                        r#"use std::fs;

                        pub fn run() {{
                            let _input = fs::read_to_string("src/inputs/day{day_name}_{part}.txt")
                                .expect("Failed to read file");
                            println!("day{day_name}_{part}");
                        }}
                    "#
                    );

                    file.write_all(boilerplate_content.as_bytes())
                        .expect("Failed to write to file");

                    Command::new("rustfmt")
                        .arg(&file_name)
                        .output()
                        .expect("Failed to run rustfmt");

                    writeln!(mod_file, "pub mod day{day_name}_{part};")
                        .expect("Failed to write to file");
                } else {
                    file.write_all("".as_bytes())
                        .expect("Failed to write to file");
                }

                println!("Created: {:?}", path);
            } else {
                println!("File already exists: {:?}", path);
            }
        }
    }
}
