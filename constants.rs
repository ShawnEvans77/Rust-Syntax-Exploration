// previous lessons: variables and mutability. 
// now, constants.
// constants. 

fn main() {
    // constants, you are not allowed to use the keyword mut. 

    let mut x = 5;

    // a constant by default is immutable, you cannot change 
    // it to mutable
    //const mut y = 10;

    // by convention, we annotate types for constants
    // based on rust rules, this expression is correct
    const Y: i32 = 10;

    const PI: f64 = 3.141592653;

    println!("The value of PI is {}", PI);
}

// you can declare a constant with a type annotation
// const PI: f64 = 3.141592653;