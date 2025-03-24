fn main() {
   let x: i32 = 400; // i8, i16, i32, i64, i128
   /*
   u8:
   2^8 unique values
   0 -2^8-1
   i8
   -2^7 - 2^7-1
   (-128 - 127)
   */
   let y: f64 = 907.54; // f32
   let c: char = 'a';
   // booleans can be represented using 1 or 0
   
   //=======( compound types
   //
   let tup: (i32, bool, char) = (2, true, 'x');// has fixed leangth
   let arr: [i32; 5] = [4, 5, 7, 8, 2];// cannot add elements
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
