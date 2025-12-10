// ownership, bowering, and references.

// ownership. 
// ---------------------------------------

// there are a number of 
// programming languages that let you control memory.

// they let you use some memory, when you finish, you release or 
// free this part of memory. 

// you might have freed memory more than once, or forgot
// to free that portion of memory. 

// C, C++ -> Memory Management Control Issue
// garbage collector -> reserving some data in memory. when the
// programmer is done dealing with this data, the garbage collector
// releases that memory. 
// this operation is done at runtime. this is one of the drawbacks
// of garbage collection. if it wants to free up the memory, 
// it stops the program. when it stops cleaning, the program
// resumes working.
// [ stopping / resuming the program]

// how has rust solved this issue?
// the white house ordered developers to move to 'memory-safe'
// langs such as rust as soon as possible!

// what is ownership?

// OWNERSHIP introduced to solve memory safety issues and high
// performance at the same time.

// what is ownership ?
// every value has a single owner [ every variable has one
// value, and it is its sole owner.]

// ownership rules
// -----------------------------------------------------
// EACH value in rust has an owner.
// there can only be one owner at a time. 
// when the owner goes out of scope, the value will be dropped. 

// example: each value in rust has a variable that is its owner.

fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of {} is {}", s1, len);

    // give the ownership of the string to the other variable.

    let s2 = s1; // we have transformed the ownership of this string
    // to a new variable called s2.

    // println!("{}", s1); < is an error. there can only be one 
    // owner at a time.

    // when the owner goes out of scope, the value will be dropped. 
    // s1 OUTSIDE OF THIS SCOPE goes out of this memory. 
}

// this gets an error.
fn printLost(s: &string) {
    println("{}", &s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}