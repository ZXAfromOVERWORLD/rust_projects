use std::error::Error;
use std::fmt;
use std::io::{self,Write};


fn main() {
    println!("Custom Error Generating Program");
    
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().to_string();

    match x.parse::<f64>() {
        Ok(num) => match calculate_sqrt(num){
            Ok(result) => println!("Square Root is {}",result),
            Err(e) => eprintln!("Error {}",e),
        },
        Err(_) => eprintln!("Invalid Number Format"),
    }
}

#[derive(Debug)]
enum MathError{
    NegativeNumber,
    PrimeNumber,
}

impl fmt::Display for MathError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match *self{
            MathError::NegativeNumber => write!(f, "Cannot Calculate the square root of a negative number."),
            MathError::PrimeNumber => write!(f, "Cannot Prime number."),
        }
    }
}

impl Error for MathError{}

fn calculate_sqrt(x : f64) -> Result<f64, MathError>{
    if x < 0.0 {
        Err(MathError::NegativeNumber)
    } else if is_prime(x){
        Err(MathError::PrimeNumber)
    } else{
        Ok(x.sqrt())
    }
}

fn is_prime(x : f64) -> bool {
    if x == 2.0 || x == 3.0 {
        return true;
    }

    for k in 1..1000{
        let p1 = 6*k + 1;
        let p2 = 6*k - 1;

        if p1 == x as usize || p2 == x as usize{
            return true;
        }
    }
    return false;
}
