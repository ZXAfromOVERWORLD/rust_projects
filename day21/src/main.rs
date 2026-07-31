use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self,Read, Write, BufReader};

const FILE : &str = "config.txt";

fn main() {
    println!("Config File Parse");
    let content = load_file(FILE);

    match content{
        Ok(cot) => parse_content(cot),
        Err(e) => eprintln!("Error Occured {}", e),
    }
}

fn load_file(x : &str) -> io::Result<String> {
    let file = OpenOptions::new().read(true).open(x)?;

    let mut reader = BufReader::new(file);

    let mut content = String::new();

    reader.read_to_string(&mut content)?;

    Ok(content)

}

fn parse_content(content : String){
    //let mut map : HashMap<String, String> =  HashMap::new();
    for line in content.lines(){
        let line = line.trim();
        if line.is_empty() || line.starts_with('#'){
            continue;
        }
        if let Some((key,value)) = line.split_once('='){
            println!("The key is {} and the value is {}", key.trim(), value.trim());
        }
    }
}
