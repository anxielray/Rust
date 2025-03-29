#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::{read_to_string, File};
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let my_tupple: (u8, String, f64) = (47, "Anxiel".to_string(), 5_000_000.00);
    println!("Name: {}", my_tupple.1);
    let(v1, v2, v3) = my_tupple;
    println!("Age: {}", v1);
}

/*
Tupples
You have to declare the types within the tupple
*/