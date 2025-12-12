// control flow in rust. 

// 1 conditions [if ... else]
// 2 repeating actions [loops]

// an if expression allows you to branch your code depending on
// conditions. 

fn main() {
    let age: u16 = 10;

    if age >= 18 {
        println!("You can drive a car!");
    } else {
        println!("You can't drive a car!");
    }

    // multiple conditions with else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statememnt
    // basically ternary operator. 
    // make sure your types properly match up.
    let condition = true;
    let number = if condition {5} else {6};
}