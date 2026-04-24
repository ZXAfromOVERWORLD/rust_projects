//Contacts Manager With Basic CRUD operations;
use std::io;
use std::process::exit;

#[derive(Debug)]
struct Contact {
    id: usize,
    name: String,
    phone: String,
    email: String,
}

impl Contact {
    fn new(idx: usize) -> Self {
        Contact {
            id: idx,
            name: prompt("Enter name of the person".to_string()),
            phone: prompt("Enter Contact".to_string()),
            email: prompt("Enter Email".to_string()),
        }
    }

    fn update_record(&mut self) {
        self.name = prompt("Enter name of the person".to_string());
        self.phone = prompt("Enter Contact".to_string());
        self.email = prompt("Enter Email".to_string())
    }

    fn view(&self) {
        println!("----------------------");
        println!("ID    : {}", self.id);
        println!("Name  : {}", self.name);
        println!("Phone : {}", self.phone);
        println!("Email : {}", self.email);
        println!("----------------------");
    }
}

fn main() {
    let mut list_contacts: Vec<Contact> = Vec::new();
    let mut next_id = 1;

    println!();
    println!("Welcome to our Contacts manager project");
    loop {
        println!();
        println!("What operation would you like to do");
        println!("1: Add New Record");
        println!("2: Update a record");
        println!("3: View all contacts");
        println!("4: Delete a contact");
        println!("5: Exit");
        println!("");
        let op = prompt("Enter Operation(1,2,..)".to_string());
        match op.as_str() {
            "1" => {
                let new_con = Contact::new(next_id);
                next_id += 1;
                list_contacts.push(new_con);
            }
            "2" => {
                let idx = prompt("Enter id you want to search".to_string())
                    .parse::<usize>()
                    .unwrap();
                for i in list_contacts.iter_mut() {
                    if i.id == idx {
                        i.update_record();
                    }
                }
            }
            "3" => {
                for i in &list_contacts {
                    i.view();
                }
            }
            "4" => {
                let idx = prompt("Enter id you want to search".to_string())
                    .parse::<usize>()
                    .unwrap();
                for i in 0..list_contacts.len() {
                    if list_contacts[i].id == idx {
                        list_contacts.remove(i);
                        break;
                    }
                }
            }
            "5" => {
                println!("Please Come Again");
                exit(0);
            }
            _ => {
                println!("Wrong Option Try Again");
                continue;
            }
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
