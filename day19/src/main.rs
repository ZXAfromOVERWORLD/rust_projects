use std::io::{self, Write};

fn main() {
    println!("State Machine");
    let mut state = State::Start;

    loop{
        match state{
            State::Start => {
                println!("WELCOME TO THE SIGNIP PAGE");
                state = State::EnterName;
            },
            State::EnterName => {
                let name = input("Enter Your Name");
                if name.is_empty(){
                    println!("Enter a valid name");
                } else {
                    state = State::EnterEmail(name);
                }
            },

            State::EnterEmail(ref name) => {
                let email = input("Enter Email");
                if email.contains("@"){
                    state = State::Confirm{name: name.clone(),email};
                } else {
                    println!("Enter a valid email must contain @");
                }
            },
            State::Confirm{name, email} => {
                println!("Please Confirm");
                println!("Your Name is {}",name);
                println!("Your Email is {}", email);
                println!("");
                let confirm = input("yes/no");
                state = match confirm.as_str(){
                    "yes" => State::Complete,
                    "no" => State::EnterName,
                    _ => {
                        println!("Invalid Choice");
                        State::Confirm{name , email}
                    }
                }
            },
            State::Complete => {
                println!("SIGNUP IS COMPLETE");
                break;
            },
        }
    }
}

enum State{
    Start,
    EnterName,
    EnterEmail(String),
    Confirm {name : String, email : String},
    Complete,
}

fn input(prompt: &str) -> String {
    print!(" {}  ", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
