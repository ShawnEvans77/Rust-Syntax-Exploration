// shadowing
// you can declare a variable with the same name as a previous
// variable
// rustaceans say the first variable is shadowed by the second.

fn main() {
    let x = 5;
    let x = x + 1; 

    // shadowing is not the same as marking a variable as 
    // mutable. 
    // because we are making a new variable with the let keyword,

    let x = x + 1;
    let x = x * 1000;

    // you are allowed to use the let keyword many times if you
    // want to use shadowing. 
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        // it says the value is 12, but in the outer scope,
        // it goes back to six. 
    }

    println!("The value of x in main function is: {x}");
}