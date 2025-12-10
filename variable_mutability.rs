// variables and mutability

// in rust, variables are immutable by default. 

// once it is declared, the variable cannot change its value by default

fn main() {
    // if you try to change it, you will have a compilation error

    let mut a: i32 = 5;
    println!("The value of a is {}", a);
    // a = 10; cannot assign twice to immutable variable 'a'
    // consider making this binding mutable

    a = 10;
}