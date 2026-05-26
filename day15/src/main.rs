use std::io::{self, BufReader, BufWriter};
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::process::exit;

const FILE : &str = "Banking.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    id : i32,
    name : String,
    balance : i32,
}

impl Account{
    fn new(id : i32) -> Self{
        let name = prompt("Enter your full name.");
        let balance = prompt("Enter the balance").parse::<i32>().unwrap();
        
        Account{
            id : id,
            name : name,
            balance: balance,
        }
        
    }

    fn view_balance(accs : &Vec<Account>){
        let id = prompt("Enter your user id").trim().parse::<i32>().unwrap();
        for i in accs{
            if i.id == id{
                println!("Your Balance is {} rupees", i.balance);
                return;
            }
        }
    }

    fn deposit(accs : &mut Vec<Account>){
        let id = prompt("Enter your user id").trim().parse::<i32>().unwrap();
        for i in accs{
            if i.id == id{
                let balance = prompt("Enter amount to depsoit").trim().parse::<i32>().unwrap();
                i.balance += balance;
                return;
            }
        }
    }

    fn withdraw(accs : &mut Vec<Account>){
        let id = prompt("Enter your user id").trim().parse::<i32>().unwrap();
        for i in accs{
            if i.id == id{
                loop{
                    let balance = prompt("Enter amount to withdraw").trim().parse::<i32>().unwrap();
                    if balance > i.balance{
                        println!("Amount to large your current balance is {}", i.balance);
                        println!("Try Again");
                        continue;
                    } else{
                        i.balance -= balance;
                        return;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut accounts: Vec<Account> = load_file();
    let mut next_id : i32 = match accounts.last(){
        Some(val) => val.id + 1,
        None => 1, 
    };

    
 
    loop {
        println!("\n🏦 Banking System:");
        println!("1. Create Account");
        println!("2. View Balance");
        println!("3. Deposit");
        println!("4. Withdraw");
        println!("5. Exit");

        match prompt("Enter which operation would you like to do").as_str(){
            "1" => {
                accounts.push(Account::new(next_id));
                next_id += 1;
            },
            "2" => Account::view_balance(&accounts),
            "3" => Account::deposit(&mut accounts),
            "4" => Account::withdraw(&mut accounts),
            "5" => {
                println!("Come again");
                save_file(accounts);
                exit(0);
            },
            _ => println!("Wrong input try again"),
        }
    }
}

fn prompt(s : &str) -> String{
    let mut var = String::new();
    println!();
    println!("{s}");
    io::stdin().read_line(&mut var).unwrap();
    var.trim().to_string()    
}

fn load_file() -> Vec<Account>{
    let file = OpenOptions::new().read(true).write(true).create(true).open(FILE).expect("Error");
    let reader = BufReader::new(file);
    
    let res : Vec<Account> = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    res
}

fn save_file(content : Vec<Account>){
    let file = OpenOptions::new().write(true).create(true).open(FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer , &content).unwrap();
}

