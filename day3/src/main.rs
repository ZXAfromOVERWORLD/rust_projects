use std::io;

fn main() {
    let mut height = String::new();
    let mut weight = String::new();

    println!("Enter your height in cm");
    io::stdin().read_line(&mut height).expect("Enter a number");
    let height : f32 = height.trim().parse().unwrap();
    
    println!("Enter your weight in kg");
    io::stdin().read_line(&mut weight).expect("Enter a number");
    let weight : f32 = weight.trim().parse().unwrap();
    
    let height : f32 = height / 100.0;

    let BMI = weight / height.powi(2);
    println!("{BMI}");
    if BMI < 18.5 {
        println!("UNDERWEIGHT");
    } else if BMI >= 18.5 && BMI <= 24.9{
        println!("NORMAL");
    } else if BMI > 24.9 && BMI < 29.9{
        println!("OVERWEIGHT");
    } else if BMI >= 30.0{
        println!("OBESE");
    } 
    



}
