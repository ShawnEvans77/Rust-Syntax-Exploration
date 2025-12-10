// functions
// the main methos is an entry point. 
// a fuction / variables should be written in snake_case.
// snake case: hello_world
// kebab case: hello-world

fn main() {
    hello_world();
    tell_height(50);
    human_id("Joel", 55, 182.3);

    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty 

        // this last line i have written here, i havent
        // written any semicolons
        // any value that evals to a certain value

        // evaluates to the last line in the expression. 

        // we also could have did return price * qty;
        // but we say this is not very "rusty."
    };

    println!("Result is: {}", x);

    let y = add(4, 6);
    println!("Value of y is: {}", y);
    println!("Value from function 'add' is: {}.", add(4, 6));

    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is {:.2}", bmi);
}

// functions can return values
fn add(a: i32, b: i32) -> i32 {
    a + b
    // again, this is the "rusty code."
}

// any variable declared outside of function should have
// the const keyword. 

fn hello_world() {
    println!("Hello, Rust!");
}

fn tell_height(height: i32) {
    println!("My height is {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {}cm.", name, age, height);
}

// expressions and statements
// expressions: anything that returns a value.
// statement: anything that does not return a value.

// expressions
// ---------------------------
// 5
// true & false
// add(3, 4)
// if condition {value1} else {value2}

// variable declerations are considered to be statements
// control flow statements - if condition, while condition, etc. 

// final example on functions:

// i want to make a function that returns the BMI.
// BMI = height(kg) / height(m)^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}