use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error");

    let input = check_valid(input.trim());

    println!("{}", check(&input));

    
}

fn check_valid(s : &str) -> String{
    let mut res = Vec::new();
    for i in s.chars(){
        if i == ' '{
            continue;
        }
        res.push(i);
    }
    let r : String = res.iter().collect();
    r
}

fn check(s: &str) -> bool{
    s == s.chars().rev().collect::<String>()
}
