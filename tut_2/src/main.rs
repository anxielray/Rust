#![allow(unused)]

//import crates
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age2 = 8;
    match age2{
        1..=18 =>println!("Important birthday!"),
        21 | 50 => println!("Important birthday!"),
        65..=i32::MAX => println!("Important birthday!"),
        _ => println!("Not an important birthday!"),//for all the rest of the ages left
    };
}

/*
To let the rust complier to assume a variable is to start the variable in an underscore
an f32 has 6 digits of precision while an f64 has 14 digits of precision
*/
