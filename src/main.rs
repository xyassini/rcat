use clap::Parser;
use std::{fs::File, io::Read};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let mut file = match File::open(&cli.path) {
        Ok(file) => file,
        Err(e) => panic!("Could not open {}: {}", cli.path.display(), e),
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => println!("{}", content),
        Err(e) => panic!("Could not read file {}: {}", cli.path.display(), e),
    }
}
