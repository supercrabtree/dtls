use std::fs;
use std::env;

fn main () {
    let args: Vec<String> = env::args().collect();
    let directory = parse_config(&args);

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        println!("\x1B[34m{}\x1B[0m", entry.path().display());
                    } else {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }
}

fn parse_config(args: &[String]) -> &str {
    let mut directory = ".";
    if args.len() == 2 {
        directory = &args[1];
    }
    directory
}
