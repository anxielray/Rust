#![allow(unused)]

//import crates
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age: u32 = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important birthday!");
    } else if (age == 21) || (age == 50) {
        println!("Important birthday!");
    } else if (age >= 65) {
        println!("Important birthday!");
    }else {
        println!("Not an important birthday");
    }
}

/*
To let the rust complier to assume a variable is to start the variable in an underscore
an f32 has 6 digits of precision while an f64 has 14 digits of precision
*/
