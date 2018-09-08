use std::fs;
use std::fs::DirEntry;
use std::env;

fn main () {
    let args: Vec<String> = env::args().collect();
    let directory = parse_config(&args);
    list_directory_contents(directory);
}

fn parse_config(args: &[String]) -> &str {
    let mut directory = ".";
    if args.len() == 2 {
        directory = &args[1];
    }
    directory
}

fn list_directory_contents(directory: &str) {
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        print_directory(entry);
                    } else {
                        print_file(entry);
                    }
                }
            }
        }
    } else {
        eprintln!("Cannot find directory {}", directory);
    }
}

fn print_directory(directory: DirEntry) {
    println!("\x1B[34m{}\x1B[0m", directory.path().display());
}

fn print_file(file: DirEntry) {
    println!("{}", file.path().display());
}
