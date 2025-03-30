use std::io;

fn is_pluperfect(n: u32) -> bool {
    let sum: u32 = n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(n.to_string().len() as u32))
        .sum();
    sum == n
}

fn main() {
    let mut input = String::new();
    println!("Enter a number to check if it's pluperfect:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: u32 = input.trim().parse().expect("Invalid number");
    if is_pluperfect(num) {
        println!("{} is a pluperfect number!", num);
    } else {
        println!("{} is NOT a pluperfect number.", num);
    }
}

