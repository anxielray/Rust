fn main() {
    let mut x = 4; // let x: u32 = 4; implicit declaration { the compiler decides the type based on the value } ( implicit type decision)
    // statically and strongly typed language ( whn you define a variable, it is given a type.)
    println!("x is: {}", x);
    x = 5;
    println!("x is: {}", x);
    // constants

    const SECONDS_TO_MINUTE: u32 = 60;
    println!(" {}", SECONDS_TO_MINUTE);
}
