fn main() {
    println!("Hello World!");
    let result = add_numbers(20, 30);

    println!("{}", result);
}
fn add_numbers(x: i32, y: i32)  -> i32{
    let result = x+y;
    if result > 10 {
        return result - 10;
    }
    result
}

/* returning a value/expression from a function
fn add_numbers(x: i32, y: i32)  -> i32{
    (x + y) or (return x+y;)
}
*/


// convention way to write the function names is snake case like in python
//statement ( let x = 20;