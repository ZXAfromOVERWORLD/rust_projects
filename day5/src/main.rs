use std::io;

fn main() {

    println!("Enter the number of digits");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error has occured");
    let input : i128 = input.trim().parse().unwrap();
    let ans = generate_fib(input);
    println!("{ans:?}");
}

pub fn generate_fib(n : i128) -> Vec<i128>{
    let mut res = Vec::new();

    if n >= 1{
        res.push(0);
    }

    if n>= 2{
        res.push(1);
    }

    for i in 2..n{
        let cur = res[i as usize - 1 ] + res[i as usize - 2];
        res.push(cur);
    }
    res
}
