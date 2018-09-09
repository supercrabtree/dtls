use std::fs;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;
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
                        print_directory(entry, metadata);
                    } else {
                        print_file(entry, metadata);
                    }
                }
            }
        }
    } else {
        eprintln!("Cannot find directory {}", directory);
    }
}

fn print_directory(directory: DirEntry, metadata: Metadata) {
    let mode = metadata.permissions().mode();
    let short_permissions = to_short_permissions(mode);
    println!("{} \x1B[34m{}\x1B[0m", short_permissions, directory.path().display());
}

fn print_file(file: DirEntry, metadata: Metadata) {
    let mode = metadata.permissions().mode();
    let short_permissions = to_short_permissions(mode);
    println!("{} {}", short_permissions, file.path().display());
}

fn to_short_permissions(mode: u32) -> String {
    let mode_as_string = String::from(format!("{:o}", mode));
    let idx = mode_as_string.len() - 3;
    let short_permissions = &mode_as_string[idx..];
    String::from(short_permissions)
}
