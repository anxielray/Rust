#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::{read_to_string, File};
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut st_1 = String::new();
    st_1.push('A');
    st_1.push_str(" nxiel");
    for word in st_1.split_whitespace(){
        println!("{}", word);
    }
    let st2 = st_1.replace("A", "Ray A");
        println!("{}", st2);
        // println!("{}", st_1);
}

/*
Strings
A string is a vector of bytes
&str is a reference to the string and allows for viewing the string
Tupples
You have to declare the types within the tupple
*/