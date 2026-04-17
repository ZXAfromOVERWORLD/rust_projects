use serde::{Deserialize, Serialize};
use std::env;
use std::fs::OpenOptions;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
    description: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Error need 1 argument");
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&args[1])
        .unwrap();

    let reader = BufReader::new(file);

    let content: Vec<User> = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    for i in content{
        println!("The ID is {}, the name of the person is {} and the role of the person is {}", i.id,i.name,i.description);
        println!();
    }
}
