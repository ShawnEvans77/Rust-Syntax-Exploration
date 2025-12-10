// primtive data types
// int, float, bool, char

// integer
// rust has signed (+ and -) and unsigned integer (only+)
// types of different sizes

// i8, i16, i32, i64, i128: signed integers
// u8, u16, u32, u64, u128: unsigned integers

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // diff between i32 (32 bits) and i64(64 bits)
    // range :
    // i32 - 2147483647
    // i64 - 9223372036854775807

    // let e: i32 = 2147483647 + 1; compiler error

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    // floats simply represent numbers with fractional parts

    // f32, f64

    let pi: f64 = 3.14;

    println!("Value of pi: {}", pi);

    // boolean values: true, false

    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);

}