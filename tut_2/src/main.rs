#![allow(unused)]

//import crates
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You gained the right to vote!"),
    };
}

/*
To let the rust complier to assume a variable is to start the variable in an underscore
an f32 has 6 digits of precision while an f64 has 14 digits of precision
*/
