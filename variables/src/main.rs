use std::io;

fn main() {
    // Constants cannot be shadowed
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds are: {THREE_HOURS_IN_SECONDS} seconds");

    // Changing the value of a mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing immutable variables
    let x = 5;
    let x = x + 1;

    // After exiting the inner scope The shadowed variable fells back to the outer scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // We are creating a new variable here so we cn give it a new type. This won't work with a mutable variable. Trying this would result in a compile-time error.
    let spaces = "   ";
    let spaces = spaces.len();

    println!("There are {spaces} spaces");

    let x = 2.0; // f64
    let y = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean datatype
    let t = true;
    let f: bool = false; // with explicit type annotation

    // char datatype
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuples cannot change size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let six_point_four = tup.1;
    println!("The value of y is: {y}");

    // Array, mostly you should use vectors
    // 5 indicates the array length
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let third = a[2];

    // Example Array index out of bound
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
