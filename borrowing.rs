// references and borrowing
// safety and performance
// borrowing and references are powerful concepts

// understanding references
// references: enable you to borrow values without taking
// ownership.
// immutable reference.
// mutable reference. 
// create reference by add "&"

// in rust, managing memory is crucial for ensuring safety
// and performance. but what does the word safety mean here?

// nobody seems to explain in detail what safety means, 
// what is safety to begin with. 

// safety refers to the prevention of certain types of bugs and
// errors that commonly occur in other languages. null pointer
// dereferencing, dangling overflows

fn main() {
    let mut _x = 5;

    // i want to create an immutable reference to the x variable.

    // let r = x; < this is transferring the ownership of r totally

    let _r = &mut _x;

    *_r += 1;
    *_r -= 3;

    println!("Value of _x: {}", _x);

    
}