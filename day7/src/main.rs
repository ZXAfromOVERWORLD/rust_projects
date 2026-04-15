use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Enter time in format: HH MM SS");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let parts: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    if parts.len() != 3 {
        println!("Please enter exactly 3 values: HH MM SS");
        return;
    }

    let mut total_seconds = parts[0] * 3600 + parts[1] * 60 + parts[2];

    for i in (0..total_seconds).rev(){
        let hours = i / 3600;
        let minutes = (i % 3600) / 60;
        let seconds = i % 60;

        print!("\rThe current time left is: {:02}:{:02}:{:02}", hours, minutes, seconds);
        io::stdout().flush().unwrap();

        sleep(Duration::from_secs(1));
    }
    println!();
    println!("Time's up!");
}
