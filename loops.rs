// repition with loops:
// doing things over and over. 

fn main() {
    // Loop keyword. an indefinite loop. basically is like
    // while true. 

    loop {
        println!("Hello, World!");
    }

    // returning a value from a counter. 
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 20 {
            break counter * 2;
        }
    };

    println!("The result is {result}!");

    // loop labels to disambiguate between multiple loops

    // the break & continue statements apply to the innermost loop
    // by default.
}