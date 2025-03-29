#![allow(unused)]

//import crates
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);
    let num_3: u32 = 5;
    let num_4: u32 = 4;
    // Initialize the random number generator
    let mut rng = rand::rng();

    // Generate a random number between 1 and 100
    let random_num: u32 = rng.random_range(1..=100);//this is an inclusive range
    println!("Random number from 1 to 100 --> {}", random_num);
}

/*
To let the rust complier to assume a variable is to start the variable in an underscore
an f32 has 6 digits of precision while an f64 has 14 digits of precision
*/
