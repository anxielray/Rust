#![allow(unused)]

//import crates
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut my_age = 18;
    let can_vote = if (my_age >= 18){
        true
    } else {
        false
    };
    println!("Status of Raym: {}", can_vote);
}

/*
To let the rust complier to assume a variable is to start the variable in an underscore
an f32 has 6 digits of precision while an f64 has 14 digits of precision
*/
