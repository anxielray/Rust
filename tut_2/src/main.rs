#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::{read_to_string, File};
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let str3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup();//remove duplicates
    for ch in v1{
        println!("{}", ch);
    }
    let st4: &str = "Random string";
    //convert  to a heap allocated string
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    //convert  a string into an array of bytes
    let byte_arr = st5.as_bytes();
    let st6 = &st5[..6];
    println!("String length: {}", st6.len());
    //delete a value ni a mutable string
    st5.clear();
    //combine strings
    let st6 = String::from("Just sum");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
}

/*
Strings
A string is a vector of bytes
&str is a reference to the string and allows for viewing the string
using &variable_string means using a copy of that string and that if we try referrencing it later we can find it. Unlike the previous, we cannot referrence a string if we use
it somewhere else without referring to its copy. it uses the string and clears it from memory, an example is when we use strings in concatenation.
Tupples
You have to declare the types within the tupple
*/