// shadowing
// you can declare a variable with the same name as a previous
// variable
// rustaceans say the first variable is shadowed by the second.

fn main() {
    let x = 5;

    let x = x + 1; 

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");

        // it says the value is 12, but in the outer scope,
        // it goes back to six. 
    }

    println!("The value of x in the outerscope is: {x}");
}