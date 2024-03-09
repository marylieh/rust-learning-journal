// The Stack can hold fix data with a fix size. This is why integers hardcoded in the sourcecode can be stored on the stack.
// This String example is different because we do not have a fixed size. Therefore we are not able to store the String onto the Stack.
// Instead we have to store the data on the Heap and use a Stack entry with a Pointer, length and capacity to locate the data on the Heap.

fn main() {
    let mut s1 = String::from("hello");
    // let mut s2 = s; // Commenting this line in throws a compile-time error because as soon as s2 is assigned the value of s1, s2 goes out of scope. This is called a move in Rust
    // This happens because we only copy the stack data and not the actual heap data. The data on the stack is: The pointer to the heap, length, capacity.

    let mut s2 = s1.clone(); // Thus works because we copy the stack data AND the actual heap data. This is can result in bad performance and a waste of memory.
    println!("{s2}");

    s1.push_str(", world!");

    println!("{}", s1);

    // Example code
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function..
                        // .. and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    // More example code
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    // Variable References
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // We can only have one mutable reference at a time.
    change(&mut s1);
    println!("The value of s1 changed to: {s1}")
} // here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

// &mut makes a reference mutable. References with & are immutable by default.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word_no_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
