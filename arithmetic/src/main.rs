use std::io;

fn main() {
   // let x = 127_000_f32;//255.0f32; // 0 -255
    // we can use the underscore for readability
    /*
    Type casting
    1. explicitly defining a type
    2. using the underscore after the value
    3. using the as keyword ... let v = x / (255 as i64)

    getting  a maximum value
    let c = (i32::MAX as i64)
    2s complimient wrapping...!
    */
   // let y = 10.0f32; // -128 - 127
    //let z = x % y;
    //println!("{}", z);
    
    //grabbing user input and converting it into an int
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let int_input: i64 = input.trim().parse().unwrap();
    // use .trim() to remove the new line input that takes the typing to the next line from the terminal, as this canniot be converted into a number
    // the .parse does the conversion of the string into an int
    //the .unwrap() picks the converted int from the string and returns it to us. Displays an error if the conversion was unsuccessful
    println!("{}", int_input);
}
