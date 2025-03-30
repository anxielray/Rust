use std::io;

fn fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = vec![0, 1];
    for i in 2..n as usize {
        let next = sequence[i - 1] + sequence[i - 2];
        sequence.push(next);
    }
    sequence
}

fn main() {
    let mut input = String::new();
    println!("Enter the number of Fibonacci terms:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Invalid number");
    let fib_sequence = fibonacci(n);
    println!("Fibonacci sequence: {:?}", fib_sequence);
}

