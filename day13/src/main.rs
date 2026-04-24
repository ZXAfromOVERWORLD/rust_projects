use std::io;
use std::thread::sleep;
use std::time::Duration;
use std::process::exit;

fn main() {
    println!();
    println!("Welcome to this String Manupulation project");
    loop {
        println!();
        println!("What operation would you like to do");
        println!("1: Reverse");
        println!("2: Uppercase");
        println!("3: Lowercase");
        println!("4: Reverse Substring");
        println!("5: Find Substring");
        println!("6: Replace Text");
        println!("7: Exit");
        println!("");
        let op = prompt("Enter Operation(1,2,..)".to_string());
        match op.as_str() {
            "1" => {
                let s = prompt("Enter String to Reverse".to_string());
                println!("Reversed String is {}", s.chars().rev().collect::<String>());
                sleep(Duration::from_secs(5));
            }
            "2" => {
                let s = prompt("Enter String in Lowercase".to_string());
                println!("Uppercase String is {}", s.to_uppercase());
                sleep(Duration::from_secs(5));
            }
            "3" => {
                let s = prompt("Enter String in Uppercase".to_string());
                println!("Lowercase String is {}", s.to_lowercase());
                sleep(Duration::from_secs(5));
            }
            "4" => {
                let mut s = prompt("Enter String".to_string());
                let start = prompt("Enter Starting".to_string());
                let end = prompt("Enter End".to_string());
                reverse_substring(&mut s, start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap());
                println!("New String is {}", s );
                sleep(Duration::from_secs(5));
            }
            "5" => {
                let s = prompt("Enter String".to_string());
                let sub= prompt("Enter Substring".to_string());
                if s.contains(&sub){
                    println!("Given String does contain {}", sub);
                } else {
                    println!("Given String does not contain {}", sub);
                }
                sleep(Duration::from_secs(5));

            }
            "6" => {
                let mut s = prompt("Enter String".to_string());
                let sub= prompt("Enter Substring to place".to_string());
                let start= prompt("Enter Starting index".to_string()).trim().parse::<usize>().unwrap();
                replace(&mut s, sub , start);
                println!("{}",s);
                sleep(Duration::from_secs(5));

            }
            "7" => {
                println!("Please Come Again");
                exit(0);
            }
            _ => {
                println!("Wrong Option Try Again");
                continue;
            },
        }
    }
}

fn prompt(s: String) -> String {
    println!();
    println!("{}", s);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn reverse_substring(s : &mut String, start : usize,mut end : usize){
    let mut v : Vec<char> = s.chars().collect();
    if end > s.len(){
        end = s.len()-1;
    }
    v[start..=end].reverse();
    *s = v.into_iter().collect();
}

fn replace(s: &mut String, sub: String, start: usize) {
   let end = sub.len() + start - 1;
   if s.len() - 1 <= end{
       *s = format!("{}{}", &s[..start], sub);
   } else {
       *s = format!("{}{}{}",&s[start..], sub, &s[end + 1..]);
   }
}
