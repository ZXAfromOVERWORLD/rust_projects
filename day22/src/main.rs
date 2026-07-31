use regex::Regex;
use std::io::{self,Write};
use std::process::exit;

fn main(){
    println!("Data Validation tool");

    loop{
        println!("Choose what to validate");
        println!("");

        println!("1) Email");
        println!("2) Phone Number");
        println!("3) Password");
        println!("4) Exit");
        
        match prompt("Enter your choice").as_str(){
            "1" => {
                let email = prompt("Enter your Email");
                if check(email){
                    println!("Your email is correct");
                } else{
                    println!("Invalid Email Email must contain @");
                }
            },
            "2" => {
                let number = prompt("Enter you phone number");
                if check_number(&number){
                    println!("Correct Number {number}");
                } else{
                    println!("Enter a valid 10 digit number");
                }
            },

            "3" => {
                let password = prompt("Enter Password");
                if check_password(&password){
                    println!("Yes this password is correct");
                } else{
                    println!("No this password is invalid");
                    println!("Password cannot contain !,_,*,(,),?,%");
                }
            },

            "4" => {
                exit(0);
            },

            _ => {
                println!("Invalid Choice please enter Again");
            }
        }
    }
}

fn check(s : String) -> bool {
    let re = Regex::new(r"^[A-Za-z0-9.-]+@[A-Za-z.]+\.(com|in|org)$").unwrap();
    re.is_match(s.as_str())
}

fn check_number(s : &str) -> bool{
    let re = Regex::new(r"^[6-9][0-9]{9}$").unwrap();
    re.is_match(s)
}

fn check_password(s : &str) -> bool{
    let re = Regex::new(r"^[A-Za-z0-9._&@#$]{8,}$").unwrap();
    re.is_match(s)
}





fn prompt(s : &str) -> String{
    println!("{s}");

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    t.trim().to_string()
}
