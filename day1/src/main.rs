use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let m_num :i32 = rand::random_range(0..=100);

    loop {
        println!("Enter your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error");

        let guess : i32 = guess.trim().parse().expect("Error");

        match guess.cmp(&m_num){
            Ordering::Less => println!("Your Guess is too small"),
            Ordering::Greater => println!("Your Guess is too big"),
            Ordering::Equal => {
                println!("Congrats you got it");
                break;
            },
        }
    }
}
