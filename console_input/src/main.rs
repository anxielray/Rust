use std::io;

fn main() {
    let mut input = String::new();// this is how we initialize a string
    io::stdin().read_line(&mut input).expect("failed to read the line");
    //create a mutable referrence to modify the value for the input
    //without the ampersand, the function will create a copy from the original inpout variable and will not work as expected
    //expect function  will handle errors and will print the message if there was an error when reading the value
    println!("{}", input);
}

/*
prelude: functionalities that a programming language adds to your project to allow
the importation of thins like the standard library functionalities
the programming languages tries to make them as minimal as possible.

packages in rust are called crates and some standard packages mini-packages are called
modules/libraries
*/
