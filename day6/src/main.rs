use serde::{Serialize, Deserialize};
use std::io::{self, BufReader, BufWriter};
use std::fs::OpenOptions;


#[derive(Serialize,Deserialize,Debug)]
struct Task{
    id : i32,
    description : String, 
    status : bool,
}




fn main() {
    let mut tasks : Vec<Task> = load_tasks();
    println!();
    println!("Welcome This is your One and Only To-Do App");
    println!();
    loop{
        println!();
        println!("Please Choose a task");
        println!();
        println!("1: View Tasks");
        println!("2: Add Tasks");
        println!("3: Modify Tasks");
        println!("4: Mark Complete");
        println!("5: Delete Tasks");
        println!("6: Exit");
        match get_input("Choose an Activity. ".to_string()).as_str(){
            "1" => view_tasks(&tasks),
            "2" => add_task(&mut tasks),
            "3" => modify_task(&mut tasks),
            "4" => complete_task(&mut tasks),
            "5" => delete_task(&mut tasks),
            "6" => {
                save_tasks(tasks);
                println!("Tasks Saved Successfully");
                break;
            },
            _ => {
                println!("Wrong input Try again");
                continue;
            },

        }
    }
}



fn load_tasks() -> Vec<Task>{
    let file = OpenOptions::new().read(true).write(true).create(true).open("tasks.json").expect("Error of SOME DIPSHIT KIND");
    let reader = BufReader::new(file);
    let res : Vec<Task> = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    res
}

fn save_tasks(tasks : Vec<Task>){

    let file = OpenOptions::new().write(true).create(true).truncate(true).open("tasks.json").expect("Error While Writing this code");
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, &tasks).expect("Error");

}

fn view_tasks(task : &Vec<Task>){
    println!();
    println!("📝 These are you tasks");
    println!();
    for i in task{
        println!();
        if i.status{
            println!("The Task id is {}",i.id);
            println!("Description: {}",i.description);
            println!("The status is Completed ✅ ");
        } else {
            println!("The Task id is {}",i.id);
            println!("Description: {}",i.description);
            println!("The status is NOT Completed ❌");
        }
    }
}

fn add_task(tasks : &mut Vec<Task>){
    let des = get_input("Enter The Description of the Task".to_string());
    let idx = tasks.len() as i32 +1;
    let cur_task = Task{
        id : idx,
        description : des,
        status : false
    };
    tasks.push(cur_task);
}

fn modify_task(tasks : &mut Vec<Task>){
    let idx = get_input("Which task to change? ID:".to_string()).trim().parse::<i32>().unwrap();
    for i in tasks{
        if i.id == idx{
            let des = get_input("Give new Description".to_string());
            i.description = des;
            return;
        }
    }
    println!("No such task exists");

}

fn delete_task(tasks : &mut Vec<Task>){
    let idx = get_input("Which task to change? ID:".to_string()).trim().parse::<i32>().unwrap();
    for i in 0..tasks.len(){
        if tasks[i].id == idx{
                tasks.remove(i);
                return;
        }
    }
    println!("Error No such Valid id");

}

fn complete_task(tasks : &mut Vec<Task>){
    let idx = get_input("Which task to change? ID:".to_string()).trim().parse::<i32>().unwrap();
    for i in tasks{
        if i.id == idx{
            i.status = true;
            return;
        }
    }
    println!("Error No such Valid id");

}




fn get_input(prompt : String) -> String{
    println!("{prompt}");
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("Error Please Enter a valid input");

    ans.trim().to_string()
}
