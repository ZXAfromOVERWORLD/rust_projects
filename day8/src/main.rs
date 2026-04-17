use rand::random_range;
use std::io;

enum GameRes{
    Win,
    Lose,
    Draw,
}

fn main() {
    println!("Welcome to the game of rock paper scissors");
    println!();
    loop{
        let user : String = get_user_input();
        
        if user == "quit".to_string(){
            println!();
            println!("Come Again Next Time");
            return;
        }
        
        let com : String = get_com_input();
        println!();
        println!("The Computer Chose {}", com);
        println!();

        match result(&user,&com){
            GameRes::Win => println!("🎉 Congrats you won"),
            GameRes::Lose => println!("😔 Oh no you lost "),
            GameRes::Draw => println!("Its a Draw, Try Again"),
        }
        println!();
    }
}

fn get_user_input() -> String{
    println!("Enter your input rock | paper | scissors | quit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str(){
        "rock" | "paper" | "scissors" | "quit" => input.trim().to_lowercase(),
        _ => {
            println!("❌  Incorrect input,  Try Again");
            get_user_input()
        },
    }
}

fn get_com_input() -> String{
    let v : Vec<&str> = vec!["rock","paper","scissors"];
    let num : usize = random_range(0..=2);
    v[num].to_string()
}

fn result(user : &str, com : &str) -> GameRes{
    
    match (user,com){
        ("rock","scissors") => GameRes::Win,
        ("paper","rock") => GameRes::Win,
        ("scissors","rock") => GameRes::Win,
        (a,b) if a==b => GameRes::Draw,
        (_,_) => GameRes::Lose,
    }
}
