#![allow(unused)]

//import crates
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_indx = 0;
    for val in arr_2.iter(){
        println!("Val: {}", val);
    }
}

/*
To let the rust complier to assume a variable is to start the variable in an underscore
an f32 has 6 digits of precision while an f64 has 14 digits of precision
arrays have a fixed size
*/
