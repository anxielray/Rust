#![allow(unused)]

//import crates
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
// use rand::Rng;
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("age wasn't assigned  a number");
    //whatever  just did from above is called shadowing; using a variable more than once. and it is acceptable to use the same variable with different data types
    age = age+1;
    println!("I'm {} and I want to earn ${}", age, ONE_MIL);
}
