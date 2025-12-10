// compound data types
// arrays, tuples, slice,s and strings (slice string)

// arrays
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);
    //let mix = [1, 2, "apple", true];

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"]; 
    println!("Fruits Array 1st element: {}", fruits[0]);

    // tuples contain a heterogenous collection 
    // of elements of fixed size.

    let human: (String, i32, bool, bool) = ("Alice".to_string(), 30, true, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices :&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Number Slice: {:?}", animal_slices);

    let book_slices :&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // Strings vs String Slices (&str)

    // Strings [ growable, mutable, owned string type]
    // Strings allocated on the Heap. Can grow or shrink in size
    // dynamically. 

    // mut keyword converts it to a 

    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");

    println!("Stone Cold Says: {:?}", stone_cold);

    // B- &str (String Slice)
    // the string slice is stored on the stack.
    // it is a 'reference'. to a portion of a string stored somewhere.

    // they are also immutable. that means you cannot modify
    // anything on them. 

    // the string slices are used to reference string literals
    // or substrings - without needing to copy or own the data.

    // these string slices are used when you want to work with
    // string data

    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];

    println!("Slice Value: {:?}", slice);
}