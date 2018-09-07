use std::fs;

fn main () {
    if let Ok(entries) = fs::read_dir(".") {
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
