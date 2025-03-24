fn main() {
    let x = 2;
    let y = x;

    example();
}

fn example() {
    let a = true;
    let b = false;
}

//Heap and stack are apart of RAM( Random Access memory )
/*
// variables are stored in RAM
RAM contains a stack and a heap
stack is faster; used to manage memory;
the normal string is immutable
dynamic string that is muttable ===( ... = String::from("Your string");
memory alocator adds values to the RAM
for heap, the allocator looks for a location that is big enough to hold the value
The heap works by storing the names of variables and the poiinters that point to the
location in the heap for access cause the elements are not organized
Stack is used to store things of fixed size, while the heap holds the dynamic varables
*/
// ownership
