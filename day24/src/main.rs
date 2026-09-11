use std::io::{self, Write};

trait Command{
    fn execute(&self);
}


struct FanOn;
struct FanOff;
struct LightOn;
struct LightOff;

impl Command for FanOn{
    fn execute(&self){
        println!("Fan Turned On");
    }
}

impl Command for FanOff{
    fn execute(&self){
        println!("Fan Turned OFF");
    }
}

impl Command for LightOff{
    fn execute(&self){
        println!("Light off");
    }
}

impl Command for LightOn{
    fn execute(&self){
        println!("Light On");
    }
}

struct Remote{
    history : Vec<String>
}

impl Remote{
    fn new() -> Self{
        Remote{history : Vec::new()}
    }

    fn press_button(&mut self, label : &str, command : &Box<dyn Command>){
        println!("Executing Command {}", label);
        command.execute();
        self.history.push(label.to_string());
    }

    fn show_history(&self){
        for i in self.history.iter(){
            println!("Command was {i}");
        }
    }
}



fn main() {
    let mut command : Box<dyn Command> = Box::new(LightOn);


    let mut remote = Remote::new();
    command = Box::new(LightOn);

    remote.press_button("Light on", &command);

}
