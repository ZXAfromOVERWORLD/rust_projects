use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
//use serder::{Serialize, Deserialize}
use std::process::exit;

const FILE: &str = "document.txt";

fn main() {
    loop {
        println!("\n📝 Text File CRUD Menu:");
        println!("1. Create (overwrite)");
        println!("2. Read");
        println!("3. Update line");
        println!("4. Delete line");
        println!("5. Exit");

        match prompt("Choose an option: ").as_str() {
            "1" => {
                overwrite();
            }
            "2" => {
                read();
            }
            "3" => {
                update();
            }
            "4" => {
                delete();
            }
            "5" => {
                exit(0);
            }
            _ => {
                println!("Error Try again");
            }
        }
    }
}

fn prompt(s: &str) -> String {
    println!("{}", s);
    let mut i = String::new();
    io::stdin().read_line(&mut i).unwrap();

    i.trim().to_string()
}

fn overwrite() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(FILE)
        .expect("error");
    let mut writer = BufWriter::new(file);

    let message = prompt("Enter New Content");
    writer.write_all(message.as_bytes()).expect("Error");
    writer.flush().unwrap();
}

fn read() {
    if let Ok(file) = OpenOptions::new().read(true).open(FILE) {
        for (i, line) in BufReader::new(file).lines().enumerate() {
            println!("{}", line.unwrap());
        }
    } else {
        println!("File not found");
    }
}

fn update() {
    let line = prompt("Enter Line No.").parse::<usize>().unwrap_or(0);
    let new = prompt("Enter The Text");
    let file = OpenOptions::new().read(true).write(true).open(FILE);

    if let Ok(file) = file {
        let mut lines: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
        if line > 0 && line <= lines.len() {
            lines[line - 1] = new;
        }

        let content = lines.join("\n") + "\n";

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(FILE)
            .unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes()).unwrap();
    }
}

fn delete() {
    let line = prompt("Enter Line No.").parse::<usize>().unwrap_or(0);
    let file = OpenOptions::new().read(true).write(true).open(FILE);

    if let Ok(file) = file {
        let mut lines: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
        if line > 0 && line <= lines.len() {
            lines.remove(line - 1);
        }

        let content = lines.join("\n") + "\n";

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(FILE)
            .unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes()).unwrap();
    }
}
