fn main() {
   let x: i32 = 400;
   let y: f64 = 907.54;
   let c: char = 'a';
   let tup: (i32, bool, char) = (2, true, 'x');
   let arr: [i32; 5] = [4, 5, 7, 8, 2];
   println!("{}", x);
   println!("{}", y);
   println!("{}", c);
   println!("{}", tup.0);
   println!("{}", tup.1);
   println!("{}", tup.2);
   println!("{}", arr[0]);
   println!("{}", arr[3]);
   //mutability must be mentioned and taken into consideration
   // the variables cannot be written out without a formatter string.
}
