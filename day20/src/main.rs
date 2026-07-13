use std::io;

fn main() {
    println!("💬 ChatBot CLI - Type 'exit' to quit");

    let mut history : Vec<String> = Vec::new();
    loop {
        let input = prompt("You: ");
        if input.contains("exit"){
            println!("Goodbye cya");
            break;
        }
        history.push(format!("You: {}",input));

        let response = bot_reply(input.as_str());
        println!("ChatBot: {}", response);

        history.push(format!("ChatBot: {}", response));

    }

    for i in history{
        println!("{}\n",i);
    }
}

fn prompt(x : &str) -> String{
    println!("{x}");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn bot_reply(s : &str) -> String{
    let s = s.to_lowercase();

    if s.contains("hello"){
        "Hi!".to_string()
    } else if s.contains("how are you"){
        "I'm doing fine and what about you".to_string()
    }else if s.contains("rust"){
        "Rust is a memory safe programming language".to_string()
    } else if s.contains("fine"){
        "Good to hear that".to_string()
    }else{
        "I dont understand that yet.".to_string()
    }
}
