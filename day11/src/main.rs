use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader};

fn main() {
    println!("Enter a file that you want to read:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let file_name = input.trim();

    let file = OpenOptions::new()
        .read(true)
        .open(file_name)
        .expect("File Not found");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(content) => println!("{}", content),
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }
}
