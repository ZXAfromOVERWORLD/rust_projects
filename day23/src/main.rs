use std::io::{self,Write};

trait Greeter{
    fn greet(&self, name: &str) -> String; 
}



struct Friendly;

impl Greeter for Friendly{
    fn greet(&self, name:&str) -> String{
        format!("Hey there {}, great to see you", name)
    }
}


struct Formal;

impl Greeter for Formal{
    fn greet(&self, name : &str) -> String{
        format!("Good day to you {}.", name )
    }
}

struct Sarcastic;

impl Greeter for Sarcastic{
    fn greet(&self, name : &str) -> String{
        format!("Oh wow, {} showed up", name)
    }
}

fn main() {
     
    let mut greeter : Box<dyn Greeter> = Box::new(Friendly);
    let name = "Siddham".to_string();
    let mut ans = greeter.greet(&name);

    println!("{ans}");

    greeter = Box::new(Sarcastic);
    ans = greeter.greet(&name);
    println!("{ans}");

}
