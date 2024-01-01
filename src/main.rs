use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'p', long = "pattern")]
    pattern: String,

    #[arg(short = 'f', long = "file")]
    path: String,
}

fn main() {
    let args = Args::parse();
    let reader = process_file(&args.path).expect("Something went wrong by processing file.");

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.to_lowercase().contains(&args.pattern.to_lowercase()) {
            println!("{}. {}", i + 1, line);
        }
    }
}

fn process_file(path: &str) -> Result<BufReader<File>, Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    Ok(reader)
}
