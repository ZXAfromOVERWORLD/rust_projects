use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::env;
use std::fs::OpenOptions;
use std::process::exit;


fn main() {
    let args :Vec<String> = env::args().collect();
    

    if args.len()!= 2 {
        println!("Error format shoudl be cargo run filemae");
        exit(0);
    }
    
    println!("Enter word to search");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Enter a word");

    let target_lowercase = s.trim().to_lowercase();

    let file = OpenOptions::new().read(true).open(&args[1]).expect("Error in opening the file");

    let mut reader = BufReader::new(file);
    
    let mut count = 0;

    for line in reader.lines(){
        let res = line.unwrap();


        let count_line = res.to_lowercase().split_whitespace().map(|x| x.trim_matches(| c : char| !c.is_alphanumeric()))
                                    .filter(|word| *word == target_lowercase).count();

        count += count_line;
    }

    println!("{count}");



}
